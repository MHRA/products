import DataLoader from 'dataloader';
import { IDocument, IDocuments } from '../../../model/document';
import { DocType, TerritoryType, docSearch } from '../../azure-search';
import { graphqlRequest } from '../../graphql';
import { convertResults } from '../../azure-results-converter';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<IProductInfo, IDocuments> => {
  return useGraphQL ? graphqlProductsLoader : azureProductsLoader;
};

export const azureProductsLoader = new DataLoader<IProductInfo, IDocuments>(
  async (searchParameterArray) => {
    return Promise.all(
      searchParameterArray.map(async (searchParameters: IProductInfo) => {
        const results = await docSearch({
          query: '',
          page: searchParameters.page,
          pageSize: searchParameters.pageSize,
          filters: {
            docType: searchParameters.docTypes,
            sortOrder: 'a-z',
            productName: searchParameters.name,
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

interface IEdge {
  node: IDocumentResponse;
}

interface IProductResponse {
  products: {
    product: {
      name: string;
      documents: { count: number; edges: IEdge[] };
    };
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
query ($productName: String!, $first: Int, $skip: Int, $docTypes: [DocumentType!], $territoryTypes: [TerritoryType!]) {
  products {
    product(name: $productName) {
      name
      documents(first: $first, offset: $skip, documentTypes: $docTypes, territoryTypes: $territoryTypes) {
        count: totalCount
        edges {
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
  }
}`;

interface IProduct {
  name: string;
  count: number;
  documents: IDocument[];
}

const convertResponseToProduct = ({
  products: {
    product: {
      name,
      documents: { count, edges },
    },
  },
}: IProductResponse): IProduct => {
  return {
    name,
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

const getDocumentsForProduct = async ({
  name,
  page,
  pageSize,
  docTypes,
  territoryTypes,
}: IProductInfo) => {
  const variables = {
    productName: name,
    first: pageSize,
    skip: calculatePageStartRecord(page, pageSize),
    docTypes: docTypes.map((s) => s.toUpperCase()),
    territoryTypes: territoryTypes.map((t) => t.toUpperCase()),
  };

  return graphqlRequest<IProductResponse, typeof variables>({
    query,
    variables,
  }).then((response) => convertResponseToProduct(response.data));
};

interface IProductInfo {
  name: string;
  page: number;
  pageSize: number;
  docTypes: DocType[];
  territoryTypes: TerritoryType[];
}

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

export const graphqlProductsLoader = new DataLoader<IProductInfo, IProduct>(
  async (productPages) => {
    return Promise.all(productPages.map(getDocumentsForProduct));
  },
);
