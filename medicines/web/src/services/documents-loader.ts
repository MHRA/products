import DataLoader from 'dataloader';
import { IDocument } from '../model/substance';
import { graphqlRequest } from './graphql';

interface IDocuments {
  count: number;
  edges: Array<{ node: IDocumentResponse }>;
}

interface IProductResponse {
  product: {
    name: string;
    documents: IDocuments;
  };
}

interface IDocumentResponse {
  product: string;
  activeSubstances: string[];
  highlights: string;
  created: string;
  docType: string;
  fileBytes: number;
  title: string;
  url: string;
}

const query = `
query ($productName: String!, $first: Int, $skip: Int) {
  product(name: $productName) {
    name
    documents(first: $first, skip: $skip) {
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
}`;

interface IProduct {
  name: string;
  count: number;
  documents: IDocument[];
}

const convertResponseToProduct = ({
  product: {
    name,
    documents: { count, edges },
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
    context: doc.highlights,
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
}: IProductPageInfo) => {
  const variables = {
    productName: name,
    first: pageSize,
    skip: calculatePageStartRecord(page, pageSize),
  };
  const { data } = await graphqlRequest<IProductResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToProduct(data);
};

interface IProductPageInfo {
  name: string;
  page: number;
  pageSize: number;
}

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

export const documents = new DataLoader<IProductPageInfo, IProduct>(
  async productPages => {
    return Promise.all(productPages.map(getDocumentsForProduct));
  },
);
