import DataLoader from 'dataloader';
import { IProduct } from '../model/substance';
import { graphqlRequest } from './graphql';

interface IResponse {
  substance: {
    products: IProductResponse[];
  };
}

interface IProductResponse {
  name: string;
  documents: { count: number };
}

const query = `
query ($substanceName: String) {
  substance(name: $substanceName) {
    products {
      name
      documents {
        count: totalCount
      }
    }
  }
}`;

const convertResponseToProduct = ({
  name,
  documents: { count },
}: IProductResponse): IProduct => {
  return {
    name,
    count,
  };
};

export const products = new DataLoader<string, IProduct[]>(
  async (substanceNames) => {
    return Promise.all(
      substanceNames.map(async (substanceName: string) => {
        const variables = { substanceName };

        const { data } = await graphqlRequest<IResponse, typeof variables>({
          query,
          variables,
        });
        console.log('PRODUCTS DATA');
        console.log(data);
        return data.substance.products.map(convertResponseToProduct);
      }),
    );
  },
);
