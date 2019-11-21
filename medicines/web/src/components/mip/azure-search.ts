const azureSearchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const azureSearchIndex = process.env.AZURE_SEARCH_INDEX;
const azureSearchKey = process.env.AZURE_SEARCH_KEY;
const azureSearchService = process.env.AZURE_SEARCH_SERVICE;
const azureSearchWordFuzziness = process.env.AZURE_SEARCH_WORD_FUZZINESS;

enum DocType {
  PilLabel,
  PilLabelAndLeaflet,
  PilLeaflet,
  Spc,
}

export interface IAzureSearchResult {
  '@search.score': number;
  '@search.highlights': { content: string[] };
  content: string;
  doc_type: DocType;
  metadata_storage_path: string;
}

const escapeSpecialCharacters = (word: string): string =>
  word.replace(/([+\-!(){}\[\]^"~*?:\/]|\|\||&&)/gi, `\\$1`);

const addAzureWordFuzziness = (word: string): string =>
  `${word}~${azureSearchWordFuzziness}`;

const buildFuzzyQuery = (query: string): string => {
  return query
    .split(' ')
    .map(word => escapeSpecialCharacters(word))
    .map(word => addAzureWordFuzziness(word))
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
