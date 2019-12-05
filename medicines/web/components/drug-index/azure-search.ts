// https://mhraproductsdev.search.windows.net/indexes/azureblob-index/docs
// ?api-key=CFBCBE8AA11AA871C14001527533870C
// &api-version=2017-11-11
// &facet=facets,count:50000,sort:value
// &$filter=facets/any(f:%20f%20eq%20%27Z%27)
// &$top=0

const azureSearchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const azureSearchIndex = process.env.AZURE_SEARCH_INDEX;
const azureSearchKey = process.env.AZURE_SEARCH_KEY;
const azureSearchService = process.env.AZURE_SEARCH_SERVICE;

export interface IAzureSearchResult {
  '@search.facets': { facets: Array<{ count: number; value: string }> };
}

const buildAzureSearchUrl = (query: string): string => {
  const url = new URL(
    `https://${azureSearchService}.search.windows.net/indexes/${azureSearchIndex}/docs`,
  );

  url.searchParams.append('api-key', azureSearchKey as string);
  url.searchParams.append('api-version', azureSearchApiVersion as string);
  url.searchParams.append('facet', 'facets,count:50000,sort:value');
  url.searchParams.append('$filter', `facets/any(f: f eq '${query}')`);
  url.searchParams.append('$top', '0');

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
  const body = await getJson(buildAzureSearchUrl(query));

  return body.value;
};
