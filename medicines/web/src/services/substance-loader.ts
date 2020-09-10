import { facetSearch } from './azure-search';

import DataLoader from 'dataloader';
import { ISubstance } from '../model/substance';
import { graphqlRequest } from './graphql';

const substanceLoader = new DataLoader<string, ISubstance[]>(async (keys) => {
  return Promise.all(keys.map(facetSearch)).then((r) =>
    r.map(([k, f]) => {
      const ss: { [id: string]: ISubstance } = {};
      f.facets
        .filter((x) => x.value.startsWith(k))
        .forEach((f) => {
          const xs = f.value.replace(/\s+/g, ' ').split(', ', 3).slice(1);
          if (xs.length > 0) {
            const s = xs[0];
            if (ss[s] === undefined) {
              if (s !== k) {
                ss[s] = { name: s, count: f.count, products: [] };
              }
            } else {
              ss[s].products?.push({ name: xs[1], count: f.count });
            }
          }
        });
      return Object.values(ss);
    }),
  );
});

interface IResponse {
  substancesByFirstLetter: Array<{
    name: string;
    products: IProductResponse[];
  }>;
}

interface IProductResponse {
  name: string;
  documents: { count: number };
}

const query = `
query ($letter: String!) {
  substancesByFirstLetter(letter: $letter) {
    name
    products {
      name
      documents {
        count: totalCount
      }
    }
  }
}`;

export const graphqlSubstanceLoader = new DataLoader<string, ISubstance[]>(
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
        console.log('RESPONSE DATA');
        console.log(data);
        return data.substancesByFirstLetter.map(({ name, products }) => {
          return {
            name,
            count: documentsCount(products),
            products: products.map(({ name, documents: { count } }) => {
              return { name, count };
            }),
          };
        });
      }),
    );
  },
);

const documentsCount = (products: IProductResponse[]) => {
  console.log('PRODUCTS!!!');
  console.log(products);
  return products.reduce(
    (total: number, { documents: { count } }) => total + count,
    0,
  );
};

export default substanceLoader;
