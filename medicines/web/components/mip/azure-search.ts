const azureSearchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const azureSearchExactnessBoost = process.env.AZURE_SEARCH_EXACTNESS_BOOST;
const azureSearchIndex = process.env.AZURE_SEARCH_INDEX;
const azureSearchKey = process.env.AZURE_SEARCH_KEY;
const azureSearchScoringProfile = process.env.AZURE_SEARCH_SCORING_PROFILE;
const azureSearchService = process.env.AZURE_SEARCH_SERVICE;
const azureSearchWordFuzziness = process.env.AZURE_SEARCH_WORD_FUZZINESS;

enum DocType {
  Par,
  PilLabel,
  PilLabelAndLeaflet,
  PilLeaflet,
  Spc,
}

export interface IAzureSearchResult {
  '@search.highlights': { content: string[] };
  '@search.score': number;
  author: string | null;
  created: string | null;
  doc_type: DocType;
  file_name: string | null;
  keywords: string | null;
  metadata_storage_name: string;
  metadata_storage_path: string;
  metadata_storage_size: number;
  release_state: string | null;
  title: string | null;
  suggestions: string[];
  substance_name: string[];
}

export interface IAzureSearchResults {
  resultCount: number;
  results: IAzureSearchResult[];
}

const escapeSpecialCharacters = (word: string): string =>
  word.replace(/([+\-!(){}\[\]^~*?:\/]|\|\||&&|AND|OR|NOT)/gi, `\\$1`);

const preferExactMatchButSupportFuzzyMatch = (word: string): string =>
  `${word}~${azureSearchWordFuzziness} ${word}^${azureSearchExactnessBoost}`;

const buildFuzzyQuery = (query: string): string => {
  return query
    .split(' ')
    .map(word => escapeSpecialCharacters(word))
    .map(word => preferExactMatchButSupportFuzzyMatch(word))
    .join(' ');
};

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

const buildAzureSearchUrl = (
  query: string,
  page: number,
  pageSize: number,
): string => {
  const url = new URL(
    `https://${azureSearchService}.search.windows.net/indexes/${azureSearchIndex}/docs`,
  );

  url.searchParams.append('api-key', azureSearchKey as string);
  url.searchParams.append('api-version', azureSearchApiVersion as string);
  url.searchParams.append('highlight', 'content');
  url.searchParams.append('queryType', 'full');
  url.searchParams.append('$count', 'true');
  url.searchParams.append('$top', `${pageSize}`);
  url.searchParams.append(
    '$skip',
    `${calculatePageStartRecord(page, pageSize)}`,
  );
  url.searchParams.append('search', query);
  url.searchParams.append(
    'scoringProfile',
    azureSearchScoringProfile as string,
  );
  // https://rb-mhra-mip.search.windows.net/indexes('azureblob-index')/docs?api-key=6D6107C925CC3C284A9218EFC626C7F7&api-version=2017-11-11&highlight=content&queryType=full&count=true&search=anti~1&$skip=100

  return url.toString();
};

const getJson = async (url: string): Promise<any> => {
  const resp: Response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (resp.ok) {
    return resp.json();
  }
};

export const azureSearch = async (
  query: string,
  page: number,
  pageSize: number,
): Promise<IAzureSearchResults> => {
  const body = await getJson(
    buildAzureSearchUrl(buildFuzzyQuery(query), page, pageSize),
  );
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};
