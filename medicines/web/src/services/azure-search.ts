import fetch, { Response } from 'node-fetch';

const searchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const searchExactnessBoost = process.env.AZURE_SEARCH_EXACTNESS_BOOST;
const searchIndex = process.env.AZURE_SEARCH_INDEX;
const searchKey = process.env.AZURE_SEARCH_KEY;
const searchScoringProfile = process.env.AZURE_SEARCH_SCORING_PROFILE;
const searchService = process.env.AZURE_SEARCH_SERVICE;
const searchWordFuzziness = process.env.AZURE_SEARCH_WORD_FUZZINESS;

enum DocType {
  Par,
  Pil,
  Spc,
}

export interface ISearchResult {
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
  product_name: string;
  release_state: string | null;
  substance_name: string[];
  suggestions: string[];
  title: string | null;
}

export interface ISearchResults {
  resultCount: number;
  results: ISearchResult[];
}

const extractProductLicenseRegExp: RegExp = new RegExp(
  '(\\b|PL)(\\s+|/|_|-)*(\\d{5})(\\s+|/|_|-)*(\\d{4})',
  'ig',
);

const escapeSpecialWords = (word: string): string =>
  word.replace(/(\|\||&&|\bAND\b|\bOR\b|\bNOT\b)/gi, `\\$1`);

const preferExactMatchButSupportFuzzyMatch = (word: string): string =>
  `${word}~${searchWordFuzziness} ${word}^${searchExactnessBoost}`;

const addNormalizedProductLicenses = (q: string): string => {
  const normalizedProductLicences = q
    .match(extractProductLicenseRegExp)
    ?.map(match => match.replace(extractProductLicenseRegExp, 'PL$3$5'));

  if (normalizedProductLicences && normalizedProductLicences.length) {
    const normalizedProductLicencesString: string = normalizedProductLicences.join(
      ' ',
    );
    return `${q} ${normalizedProductLicencesString}`;
  }

  return `${q}`;
};

const splitByNonSearchableCharacters = (query: string) =>
  query.split(/(?:[,+\-!(){}\[\]^~*?:\/]|\s+)/gi);

const buildFuzzyQuery = (query: string): string => {
  return splitByNonSearchableCharacters(addNormalizedProductLicenses(query))
    .map(word => escapeSpecialWords(word))
    .map(word => preferExactMatchButSupportFuzzyMatch(word))
    .join(' ');
};

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

const buildSearchUrl = (
  query: string,
  page: number,
  pageSize: number,
): string => {
  const url = new URL(
    `https://${searchService}.search.windows.net/indexes/${searchIndex}/docs`,
  );

  url.searchParams.append('api-key', searchKey as string);
  url.searchParams.append('api-version', searchApiVersion as string);
  url.searchParams.append('highlight', 'content');
  url.searchParams.append('queryType', 'full');
  url.searchParams.append('$count', 'true');
  url.searchParams.append('$top', `${pageSize}`);
  url.searchParams.append(
    '$skip',
    `${calculatePageStartRecord(page, pageSize)}`,
  );
  url.searchParams.append('search', query);
  url.searchParams.append('scoringProfile', searchScoringProfile as string);
  url.searchParams.append('searchMode', 'all');

  return url.toString();
};

const buildProductSearchUrl = (
  product: string,
  page: number,
  pageSize: number,
): string => {
  const url = new URL(
    `https://${searchService}.search.windows.net/indexes/${searchIndex}/docs`,
  );

  url.searchParams.append('api-key', searchKey as string);
  url.searchParams.append('api-version', searchApiVersion as string);
  url.searchParams.append('highlight', 'content');
  url.searchParams.append('queryType', 'full');
  url.searchParams.append('$count', 'true');
  url.searchParams.append('$top', `${pageSize}`);
  url.searchParams.append(
    '$skip',
    `${calculatePageStartRecord(page, pageSize)}`,
  );
  url.searchParams.append('$filter', "product_name eq '" + product + "'");

  return url.toString();
};

export interface IFacetResult {
  facets: Array<{ count: number; value: string }>;
}

const buildFacetUrl = (query: string): string => {
  const url = new URL(
    `https://${searchService}.search.windows.net/indexes/${searchIndex}/docs`,
  );

  url.searchParams.append('api-key', searchKey as string);
  url.searchParams.append('api-version', searchApiVersion as string);
  url.searchParams.append('facet', 'facets,count:50000,sort:value');
  url.searchParams.append('$filter', `facets/any(f: f eq '${query}')`);
  url.searchParams.append('$top', '0');
  url.searchParams.append('searchMode', 'all');

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

export const docSearch = async (
  query: string,
  page: number,
  pageSize: number,
): Promise<ISearchResults> => {
  const body = await getJson(
    buildSearchUrl(buildFuzzyQuery(query), page, pageSize),
  );
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};

export const productSearch = async (
  product: string,
  page: number,
  pageSize: number,
): Promise<ISearchResults> => {
  const body = await getJson(buildProductSearchUrl(product, page, pageSize));
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};

export const facetSearch = async (
  query: string,
): Promise<[string, IFacetResult]> => {
  const body = await getJson(buildFacetUrl(query));

  return [query, body['@search.facets']];
};
