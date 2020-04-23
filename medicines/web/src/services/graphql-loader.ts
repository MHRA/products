import DataLoader from 'dataloader';
import { IProduct } from '../model/substance';

const graphQlUrl = process.env.GRAPHQL_URL as string;

const getProductsForSubstance = async (substanceName: string) => {
  const query = `
query ($substanceName: String) {
  substance(name: $substanceName) {
    products {
      name
      documentCount
    }
  }
}`;

  const variables = { substanceName };

  const response = await fetch(graphQlUrl, {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      query,
      variables,
    }),
  });

  return (await response.json()).data.substance.products;
};

const products = new DataLoader<string, IProduct[]>(async substanceNames => {
  return Promise.all(substanceNames.map(getProductsForSubstance));
});

export default {
  products,
};
