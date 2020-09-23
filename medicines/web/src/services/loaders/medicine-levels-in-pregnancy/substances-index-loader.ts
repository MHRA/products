import { bmgfFacetSearch, IFacetResult } from '../../azure-search';

import DataLoader from 'dataloader';
import { ISubstanceIndex } from '../../../model/substance';
import { graphqlRequest } from '../../graphql';

const mapSubstance = ([letter, facetResult]: [
  string,
  IFacetResult,
]): ISubstanceIndex[] => {
  const indexResults: { [id: string]: ISubstanceIndex } = {};
  facetResult.facets
    .filter((x) => x.value.startsWith(letter))
    .forEach((f) => {
      const [substance] = f.value.replace(/\s+/g, ' ').split(', ', 3).slice(1);

      if (substance) {
        indexResults[substance] = { name: substance, count: f.count };
      }
    });
  return Object.values(indexResults);
};

export const substancesIndexLoader = new DataLoader<string, ISubstanceIndex[]>(
  async (substanceLetters) => {
    return Promise.all(substanceLetters.map(bmgfFacetSearch)).then((r) =>
      r.map(mapSubstance),
    );
  },
);

interface IResponse {
  medicineLevelsInPregnancy: { substancesIndex: ISubstanceIndex[] };
}

const bmgfQuery = `
query ($letter: String!) {
  medicineLevelsInPregnancy {
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
        query: bmgfQuery,
        variables,
      });

      return data.medicineLevelsInPregnancy.substancesIndex;
    }),
  );
});
