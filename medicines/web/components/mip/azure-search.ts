// @ts-ignore
import URL from 'url-polyfill';

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

const escapeSpecialCharacters = (word: string): string =>
  word.replace(/([+\-!(){}\[\]^"~*?:\/]|\|\||&&)/gi, `\\$1`);

const preferExactMatchButSupportFuzzyMatch = (word: string): string =>
  `${word}~${azureSearchWordFuzziness} ${word}^${azureSearchExactnessBoost}`;

const buildFuzzyQuery = (query: string): string => {
  return query
    .split(' ')
    .map(word => escapeSpecialCharacters(word))
    .map(word => preferExactMatchButSupportFuzzyMatch(word))
    .join(' ');
};

const buildAzureSearchUrl = (query: string): string => {
  const url = new URL(
    `https://${azureSearchService}.search.windows.net/indexes/${azureSearchIndex}/docs`,
  );

  url.searchParams.append('api-key', azureSearchKey as string);
  url.searchParams.append('api-version', azureSearchApiVersion as string);
  url.searchParams.append('highlight', 'content');
  url.searchParams.append('queryType', 'full');
  url.searchParams.append('search', query);
  url.searchParams.append(
    'scoringProfile',
    azureSearchScoringProfile as string,
  );

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
): Promise<IAzureSearchResult[]> => {
  const body = await getJson(buildAzureSearchUrl(buildFuzzyQuery(query)));

  return body.value;
};
