import { bmgfFacetSearch } from '../../azure-search';

import DataLoader from 'dataloader';
import { IFacet } from '../../../model/facet';
import { graphqlRequest } from '../../graphql';
import { mapSubstancesIndex } from '../../azure-results-converter';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<string, IFacet[]> => {
  return useGraphQL ? graphqlSubstancesIndexLoader : azureSubstancesIndexLoader;
};

export const azureSubstancesIndexLoader = new DataLoader<string, IFacet[]>(
  async (substanceLetters) => {
    return Promise.all(substanceLetters.map(bmgfFacetSearch)).then((r) =>
      r.map(mapSubstancesIndex),
    );
  },
);

interface IResponse {
  medicineLevelsInPregnancy: { substancesIndex: IFacet[] };
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

export const graphqlSubstancesIndexLoader = new DataLoader<string, IFacet[]>(
  async (keys) => {
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
  },
);
