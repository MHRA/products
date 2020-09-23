import { facetSearch, IFacetResult } from '../../azure-search';

import DataLoader from 'dataloader';
import { ISubstance, ISubstanceIndex } from '../../../model/substance';
import { graphqlRequest } from '../../graphql';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<string, ISubstance[]> => {
  return useGraphQL ? graphqlSubstancesIndexLoader : azureSubstancesIndexLoader;
};

export const azureSubstancesIndexLoader = new DataLoader<string, ISubstance[]>(
  async (keys) => {
    return Promise.all(keys.map(facetSearch)).then((r) => r.map(mapSubstance));
  },
);

const mapSubstance = ([keyToLoad, facetResult]: [
  string,
  IFacetResult,
]): ISubstance[] => {
  const indexResults: { [id: string]: ISubstance } = {};
  facetResult.facets
    .filter((facet) => facet.value.startsWith(keyToLoad))
    .forEach((facet) => {
      const [substance, product] = facet.value
        .replace(/\s+/g, ' ')
        .split(', ', 3)
        .slice(1);
      if (substance) {
        const substanceIsInResults = indexResults[substance] !== undefined;
        const substanceShouldBeInResults = substance !== keyToLoad;

        if (substanceIsInResults) {
          indexResults[substance].products?.push({
            name: product,
            count: facet.count,
          });
        } else if (substanceShouldBeInResults) {
          indexResults[substance] = {
            name: substance,
            count: facet.count,
            products: [],
          };
        }
      }
    });
  return Object.values(indexResults);
};

interface IResponse {
  products: { substancesIndex: ISubstanceIndex[] };
}

const query = `
query ($letter: String!) {
  products {
    substancesIndex(letter: $letter) {
      name
      count
    }
  }
}`;

export const graphqlSubstancesIndexLoader = new DataLoader<
  string,
  ISubstanceIndex[]
>(async (keys) => {
  return Promise.all(
    // Could potentially batch the queries for all of the keys into one GraphQL request but there's never
    // actually a request for more than one at a time yet so no point in implementing that yet
    keys.map(async (letter) => {
      const variables = { letter };

      const { data } = await graphqlRequest<IResponse, typeof variables>({
        query,
        variables,
      });

      return data.products.substancesIndex;
    }),
  );
});
