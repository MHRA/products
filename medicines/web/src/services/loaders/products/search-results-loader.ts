import DataLoader from 'dataloader';
import { IDocument, IDocuments } from '../../../model/document';
import { DocType, docSearch } from '../../azure-search';
import { graphqlRequest } from '../../graphql';
import { convertResults } from '../../azure-results-converter';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<ISearchInfo, IDocuments> => {
  return useGraphQL ? graphqlSearchLoader : azureSearchLoader;
};

export const azureSearchLoader = new DataLoader<ISearchInfo, IDocuments>(
  async (searchParameterArray) => {
    return Promise.all(
      searchParameterArray.map(async (searchParameters: ISearchInfo) => {
        const results = await docSearch({
          query: searchParameters.searchTerm,
          page: searchParameters.page,
          pageSize: searchParameters.pageSize,
          filters: {
            docType: searchParameters.docTypes,
            sortOrder: 'a-z',
          },
        });

        return {
          count: results.resultCount,
          documents: results.results.map(convertResults),
        };
      }),
    );
  },
);

interface ISearchInfo {
  searchTerm: string;
  page: number;
  pageSize: number;
  docTypes: DocType[];
}

interface IEdge {
  node: IDocumentResponse;
}

interface ISearchResponse {
  products: {
    documents: { count: number; edges: IEdge[] };
  };
}

interface IDocumentResponse {
  product: string;
  activeSubstances: string[];
  highlights: string[];
  created: string;
  docType: string;
  fileBytes: number;
  title: string;
  url: string;
}

const query = `
query($searchTerm: String, $first: Int, $after: String, $documentTypes: [DocumentType!]) {
  products {
    documents(search: $searchTerm, first: $first, after: $after, documentTypes: $documentTypes) {
      count: totalCount
      edges {
        cursor
        node {
          product: productName
          activeSubstances
          highlights
          created
          docType
          fileBytes: fileSizeInBytes
          title
          url
        }
      }
    }
  }
}`;

const convertResponseToSearchPage = ({
  products: {
    documents: { count, edges },
  },
}: ISearchResponse): IDocuments => {
  return {
    count,
    documents: edges.map(convertDocumentResponseToDocument),
  };
};

const convertDocumentResponseToDocument = ({
  node: doc,
}: {
  node: IDocumentResponse;
}): IDocument => {
  return {
    activeSubstances: doc.activeSubstances,
    context: doc.highlights?.join(' … ') || '',
    created: doc.created,
    docType: doc.docType,
    fileSize: Math.ceil(doc.fileBytes / 1000).toLocaleString('en-GB'),
    name: doc.title,
    product: doc.product,
    url: doc.url,
  };
};

const getDocumentsForSearch = async ({
  searchTerm,
  page,
  pageSize,
  docTypes,
}: ISearchInfo) => {
  const variables = {
    searchTerm,
    first: pageSize,
    after: makeCursor(page, pageSize),
    documentTypes: docTypes.map((s) => s.toUpperCase()),
  };
  const { data } = await graphqlRequest<ISearchResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToSearchPage(data);
};

export const makeCursor = (page: number, pageSize: number): string => {
  const skip = calculatePageStartRecord(page, pageSize);

  return Buffer.from((skip - 1).toString()).toString('base64');
};

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

export const graphqlSearchLoader = new DataLoader<ISearchInfo, IDocuments>(
  async (searchTerms) => {
    return Promise.all(searchTerms.map(getDocumentsForSearch));
  },
);
