import fetch, { Response } from 'node-fetch';
import { requestTimeout } from './request-helper';
import { buildFuzzyQuery } from './search-query-normalizer';

const searchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const productsSearchIndex = process.env.AZURE_SEARCH_INDEX || '';
const bmgfSearchIndex = process.env.BMGF_AZURE_SEARCH_INDEX || '';
const searchKey = process.env.AZURE_SEARCH_KEY;
const searchScoringProfile = process.env.AZURE_SEARCH_SCORING_PROFILE;
const searchService = process.env.AZURE_SEARCH_SERVICE;
const requestTimeoutMs: number = 15000;

export enum DocType {
  Par = 'Par',
  Pil = 'Pil',
  Spc = 'Spc',
}

export enum TerritoryType {
  UK = 'UK',
  NI = 'NI',
  GB = 'GB',
}

export enum SearchType {
  Doc = 'doc',
  Territory = 'ter',
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

export interface IBmgfSearchResult {
  '@search.highlights': { content: string[] };
  '@search.score': number;
  file_name: string | null;
  metadata_storage_name: string;
  metadata_storage_path: string;
  metadata_storage_size: number;
  products: string[];
  active_substances: string[];
  summary: string;
  pbpk_models: string[];
  matrices: string[];
  pl_numbers: string[];
  report_name: string | null;
  pregnancy_trimesters: string[];
}

export interface IBmgfSearchResults {
  resultCount: number;
  results: IBmgfSearchResult[];
}

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

const buildSearchUrl = (
  query: string,
  page: number,
  pageSize: number,
  index: string,
  filters: ISearchFilters,
): string => {
  const url = buildBaseUrl(index);
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
  addFilterParameter(url, filters);

  return url.toString();
};

const addFilterParameter = (url: URL, filters: ISearchFilters) => {
  const filterParameter = createFilter(filters);
  if (filterParameter.length > 0) {
    url.searchParams.append('$filter', filterParameter);
  }
};

interface IFacet {
  count: number;
  value: string;
}

export interface IFacetResult {
  facets: IFacet[];
}

const buildBaseUrl = (index: string): URL => {
  const url = new URL(
    `https://${searchService}.search.windows.net/indexes/${index}/docs`,
  );

  url.searchParams.append('api-key', searchKey as string);
  url.searchParams.append('api-version', searchApiVersion as string);
  return url;
};

const buildFacetUrl = (query: string, index: string): string => {
  const url = buildBaseUrl(index);
  url.searchParams.append('facet', 'facets,count:50000,sort:value');
  url.searchParams.append('$filter', `facets/any(f: f eq '${query}')`);
  url.searchParams.append('$top', '0');
  url.searchParams.append('searchMode', 'all');

  return url.toString();
};

const getJson = async (url: string): Promise<any> => {
  const resp: Response = await requestTimeout(
    requestTimeoutMs,
    fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    }),
  );

  if (resp.ok) {
    return resp.json();
  }
};

export interface ISearchFilters {
  docType?: DocType[];
  territoryType?: TerritoryType[];
  substanceName?: string;
  productName?: string;
  sortOrder: string;
}

interface ISearchQuery {
  query: string;
  page: number;
  pageSize: number;
  filters: ISearchFilters;
}

export const docSearch = async (
  query: ISearchQuery,
): Promise<ISearchResults> => {
  const body = await getJson(
    buildSearchUrl(
      buildFuzzyQuery(query.query),
      query.page,
      query.pageSize,
      productsSearchIndex,
      query.filters,
    ),
  );
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};

export const bmgfDocSearch = async (
  query: ISearchQuery,
): Promise<IBmgfSearchResults> => {
  const body = await getJson(
    buildSearchUrl(
      buildFuzzyQuery(query.query),
      query.page,
      query.pageSize,
      bmgfSearchIndex,
      query.filters,
    ),
  );
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};

export const facetSearch = async (
  query: string,
): Promise<[string, IFacetResult]> => {
  const body = await getJson(buildFacetUrl(query, productsSearchIndex));
  return [query, body['@search.facets']];
};

export const bmgfFacetSearch = async (
  query: string,
): Promise<[string, IFacetResult]> => {
  const body = await getJson(buildFacetUrl(query, bmgfSearchIndex));
  return [query, body['@search.facets']];
};

const createFilter = (filters: ISearchFilters) => {
  const filterParams: string[] = [];
  if (filters.docType && filters.docType.length > 0) {
    const docTypeFilters = [];
    for (const docType of filters.docType) {
      docTypeFilters.push(`doc_type eq '${docType}'`);
    }
    filterParams.push('(' + docTypeFilters.join(' or ') + ')');
  }
  if (filters.territoryType && filters.territoryType.length > 0) {
    const territoryTypeFilters = [];
    for (const territoryType of filters.territoryType) {
      if ([TerritoryType.GB, TerritoryType.NI].includes(territoryType)) {
        territoryTypeFilters.push(`territory eq '${territoryType}'`);
      }
    }
    territoryTypeFilters.push("territory eq 'UK'");
    territoryTypeFilters.push('territory eq null');

    filterParams.push('(' + territoryTypeFilters.join(' or ') + ')');
  }
  if (filters.substanceName) {
    filterParams.push(
      `active_substances/any(substance: substance eq '${filters.substanceName.toUpperCase()}')`,
    );
  }
  if (filters.productName) {
    filterParams.push(`product_name eq '${filters.productName.toUpperCase()}'`);
  }
  return filterParams.join(' and ');
};
