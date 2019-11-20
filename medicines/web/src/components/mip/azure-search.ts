const azureIndex = 'azureblob-index';
const azureService = 'rb-mhra-mip';
const azureApiVersion = '2017-11-11';
const azureKey = '6D6107C925CC3C284A9218EFC626C7F7';

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

const buildAzureSearchUrl = (query: string): string => {
  const url = new URL(
    `https://${azureService}.search.windows.net/indexes/${azureIndex}/docs`,
  );

  url.searchParams.append('api-key', azureKey as string);
  url.searchParams.append('api-version', azureApiVersion as string);
  url.searchParams.append('search', query);
  url.searchParams.append('highlight', 'content');

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
