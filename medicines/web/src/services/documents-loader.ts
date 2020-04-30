import DataLoader from 'dataloader';
import { IDocument } from '../model/substance';
import { graphqlRequest } from './graphql';

interface IDocuments {
  count: number;
  edges: Array<{ node: IDocument }>;
}

interface IProductResponse {
  name: string;
  documents: IDocuments;
}

const query = `
query ($productName: String!) {
  product(name: $productName) {
    name
    documents {
      count: totalCount
      edges {
        node {
          productName
          activeSubstances
          title
          highlights
          created
          docType
          fileBytes: fileSizeInBytes
          name
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
  name,
  documents: { count, edges },
}: IProductResponse): IProduct => {
  return {
    name,
    count,
    documents: edges.map(x => x.node),
  };
};

const getDocumentsForProduct = async (productName: any) => {
  const variables = { productName };
  const { data } = await graphqlRequest<IProductResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToProduct(data);
};

export const documents = new DataLoader<string, IProduct>(
  async productNames => {
    return Promise.all(productNames.map(getDocumentsForProduct));
  },
);
