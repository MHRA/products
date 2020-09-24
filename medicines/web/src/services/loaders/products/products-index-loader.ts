import DataLoader from 'dataloader';
import { IFacet } from '../../../model/facet';
import { graphqlRequest } from '../../graphql';
import { facetSearch } from '../../azure-search';
import { mapProductsIndex } from '../../azure-results-converter';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<string, IFacet[]> => {
  return useGraphQL ? graphqlProductsIndexLoader : azureProductsIndexLoader;
};

export const azureProductsIndexLoader = new DataLoader<string, IFacet[]>(
  async (substanceNames) => {
    return Promise.all(
      substanceNames.map(async (substanceName: string) =>
        facetSearch(substanceName.charAt(0)).then((r) => {
          const [letter, result] = r;
          return mapProductsIndex([`${letter}, ${substanceName}`, result]);
        }),
      ),
    );
  },
);

interface IResponse {
  products: {
    productsIndex: IFacet[];
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

export const graphqlProductsIndexLoader = new DataLoader<string, IFacet[]>(
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
