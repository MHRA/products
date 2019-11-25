const azureSearchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const azureSearchIndex = process.env.AZURE_SEARCH_INDEX;
const azureSearchKey = process.env.AZURE_SEARCH_KEY;
const azureSearchService = process.env.AZURE_SEARCH_SERVICE;
const azureSearchSuggester = process.env.AZURE_SEARCH_SUGGESTER;
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
}

export interface IAzureSuggestion {
  '@search.text': string;
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

const buildDocumentSearchUrl = (query: string): string => {
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

const buildSuggestionsSearchUrl = (query: string): string => {
  const url = new URL(
    `https://${azureSearchService}.search.windows.net/indexes/${azureSearchIndex}/docs/suggest`,
  );

  url.searchParams.append('$top', '10');
  url.searchParams.append('api-version', azureSearchApiVersion as string);
  url.searchParams.append('fuzzy', 'true');
  url.searchParams.append('search', query);
  url.searchParams.append('suggesterName', azureSearchSuggester as string);

  return url.toString();
};

const getJson = async (
  url: string,
  headers: { [key: string]: string } = {},
): Promise<any> => {
  const resp: Response = await fetch(url, {
    method: 'GET',
    headers: {
      ...headers,
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
  const body = await getJson(buildDocumentSearchUrl(buildFuzzyQuery(query)));

  return body.value;
};

export const getSuggestions = async (
  query: string,
): Promise<IAzureSuggestion[]> => {
  const body = await getJson(buildSuggestionsSearchUrl(query), {
    'api-key': azureSearchKey as string,
  });

  return body.value;
};
