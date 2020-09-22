import DataLoader from 'dataloader';
import { IDocument } from '../../../model/document';
import { DocType } from '../../azure-search';
import { graphqlRequest } from '../../graphql';

interface IEdge {
  node: IDocumentResponse;
}

interface IDocuments {
  count: number;
  edges: IEdge[];
}

interface IProductResponse {
  products: {
    product: {
      name: string;
      documents: IDocuments;
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
query ($productName: String!, $first: Int, $skip: Int, $docTypes: [DocumentType!]) {
  products {
    product(name: $productName) {
      name
      documents(first: $first, offset: $skip, documentTypes: $docTypes) {
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
}: IProductPageInfo) => {
  const variables = {
    productName: name,
    first: pageSize,
    skip: calculatePageStartRecord(page, pageSize),
    docTypes: docTypes.map((s) => s.toUpperCase()),
  };

  return graphqlRequest<IProductResponse, typeof variables>({
    query,
    variables,
  }).then((response) => convertResponseToProduct(response.data));
};

interface IProductPageInfo {
  name: string;
  page: number;
  pageSize: number;
  docTypes: DocType[];
}

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

export const documents = new DataLoader<IProductPageInfo, IProduct>(
  async (productPages) => {
    return Promise.all(productPages.map(getDocumentsForProduct));
  },
);
