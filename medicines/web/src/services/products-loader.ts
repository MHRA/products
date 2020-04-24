import DataLoader from 'dataloader';
import { IProduct } from '../model/substance';
import { graphqlRequest } from './graphql';

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

  const resp = await graphqlRequest<
    { substance: { products: IProduct[] } },
    typeof variables
  >({ query, variables });

  return resp.data.substance.products;
};

export const products = new DataLoader<string, IProduct[]>(
  async substanceNames => {
    return Promise.all(substanceNames.map(getProductsForSubstance));
  },
);
