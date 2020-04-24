import DataLoader from 'dataloader';
import { IDocument } from '../model/substance';

const graphQlUrl = process.env.GRAPHQL_URL as string;

const fetchFromGraphQl = (query: string): Promise<Response> => {
  return fetch(graphQlUrl, {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ query, variables: null }),
  });
};

const getDocumentsForProduct = async (productName: any) => {
  const query = `{ product(name: "${productName}") { documents {       productName
    activeSubstances
    title
    highlights
    created
    docType
    fileBytes
    name
    url } } }`;
  const response = await fetchFromGraphQl(query);

  return (await response.json()).data.product.documents;
};

export const documents = new DataLoader<string, IDocument[]>(
  async productNames => {
    return Promise.all(productNames.map(getDocumentsForProduct));
  },
);
