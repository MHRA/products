import { facetSearch, IFacetResult } from '../../azure-search';

import DataLoader from 'dataloader';
import { ISubstance, ISubstanceIndex } from '../../../model/substance';
import { graphqlRequest } from '../../graphql';

const substanceLoader = new DataLoader<string, ISubstance[]>(async (keys) => {
  return Promise.all(keys.map(facetSearch)).then((r) => r.map(mapSubstance));
});

const mapSubstance = ([key, facetResult]: [
  string,
  IFacetResult,
]): ISubstance[] => {
  const ss: { [id: string]: ISubstance } = {};
  facetResult.facets
    .filter((x) => x.value.startsWith(key))
    .forEach((f) => {
      const xs = f.value.replace(/\s+/g, ' ').split(', ', 3).slice(1);
      if (xs.length > 0) {
        const s = xs[0];
        if (ss[s] === undefined) {
          if (s !== key) {
            ss[s] = { name: s, count: f.count, products: [] };
          }
        } else {
          ss[s].products?.push({ name: xs[1], count: f.count });
        }
      }
    });
  return Object.values(ss);
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

export const graphqlSubstanceLoader = new DataLoader<string, ISubstanceIndex[]>(
  async (keys) => {
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
  },
);

export default substanceLoader;
