import DataLoader from 'dataloader';
import { IProduct } from '../../../model/product';
import { graphqlRequest } from '../../graphql';
import { azureSubstancesIndexLoader } from './substances-index-loader';

interface IProductIndexItem {
  name: string;
  count: number;
}

interface IResponse {
  products: {
    productsIndex: IProductIndexItem[];
  };
}

const query = `
query ($substance: String) {
  products {
    productsIndex(substance: $substance) {
      name
      count
    }
  }
}`;

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<string, IProductIndexItem[]> => {
  return useGraphQL ? graphqlProductsIndexLoader : azureProductsIndexLoader;
};

export const azureProductsIndexLoader = new DataLoader<
  string,
  IProductIndexItem[]
>(async (substanceNames) => {
  return Promise.all(
    substanceNames.map(async (substanceName: string) => {
      const substancesIndex = await azureSubstancesIndexLoader.load(
        substanceName.charAt(0),
      );
      const substanceMatch = substancesIndex.find(
        (substance) => substance.name === substanceName,
      );
      if (substanceMatch && substanceMatch.products?.length) {
        return substanceMatch.products as IProductIndexItem[];
      }
      return [];
    }),
  );
});

export const graphqlProductsIndexLoader = new DataLoader<
  string,
  IProductIndexItem[]
>(async (substanceNames) => {
  return Promise.all(
    substanceNames.map(async (substanceName: string) => {
      const variables = { substance: substanceName };

      const response = await graphqlRequest<IResponse, typeof variables>({
        query,
        variables,
      });

      if (!response.data) {
        return [];
      }

      return response.data.products.productsIndex;
    }),
  );
});
