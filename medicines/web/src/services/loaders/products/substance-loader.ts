import DataLoader from 'dataloader';
import { IProduct } from '../../../model/product';
import { graphqlRequest } from '../../graphql';

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

export const graphqlProductsLoader = new DataLoader<string, IProduct[]>(
  async (substanceNames) => {
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
  },
);
