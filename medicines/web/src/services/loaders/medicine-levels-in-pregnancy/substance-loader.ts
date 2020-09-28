import DataLoader from 'dataloader';
import { IBmgfReport, IBmgfReports } from '../../../model/document';
import {
  IEdge,
  convertReportResponseToReport,
  calculatePageStartRecord,
} from './search-results-loader';
import { bmgfDocSearch } from '../../azure-search';
import { graphqlRequest } from '../../graphql';
import { convertBmgfResults } from '../../azure-results-converter';

export const getLoader = (
  useGraphQL: boolean,
): DataLoader<ISubstanceInfo, IBmgfReports> => {
  return useGraphQL ? graphqlSubstanceLoader : azureSubstanceLoader;
};

export const azureSubstanceLoader = new DataLoader<
  ISubstanceInfo,
  IBmgfReports
>(async (searchParameterArray) => {
  return Promise.all(
    searchParameterArray.map(async (searchParameters: ISubstanceInfo) => {
      const results = await bmgfDocSearch({
        query: '',
        page: searchParameters.page,
        pageSize: searchParameters.pageSize,
        filters: {
          sortOrder: 'a-z',
          substanceName: searchParameters.name,
        },
      });

      return {
        count: results.resultCount,
        reports: results.results.map(convertBmgfResults),
      };
    }),
  );
});

interface ISubstanceResponse {
  medicineLevelsInPregnancy: {
    substance: {
      name: string;
      reports: {
        count: number;
        edges: IEdge[];
      };
    };
  };
}

export interface ISubstance {
  name: string;
  count: number;
  reports: IBmgfReport[];
}

export interface ISubstanceInfo {
  name: string;
  page: number;
  pageSize: number;
}

const query = `
query ($substanceName: String!, $first: Int, $skip: Int) {
  medicineLevelsInPregnancy {
    substance(name: $substanceName) {
      name
      reports(first: $first, offset: $skip) {
        count: totalCount
        edges {
          node {
            products
            activeSubstances
            title
            highlights
            fileSizeInBytes
            fileName
            fileUrl
            summary
            matrices
            pbpkModels
          }
        }
      }
    }
  }
}`;

export const graphqlSubstanceLoader = new DataLoader<
  ISubstanceInfo,
  ISubstance
>(async (substances) => {
  return Promise.all(substances.map(getReportsForSubstance));
});

const getReportsForSubstance = async ({
  name,
  page,
  pageSize,
}: ISubstanceInfo) => {
  const variables = {
    substanceName: name,
    first: pageSize,
    skip: calculatePageStartRecord(page, pageSize),
  };

  const { data } = await graphqlRequest<ISubstanceResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToSubstance(data);
};

const convertResponseToSubstance = ({
  medicineLevelsInPregnancy: {
    substance: {
      name,
      reports: { count, edges },
    },
  },
}: ISubstanceResponse): ISubstance => {
  return {
    name,
    count,
    reports: edges.map(convertReportResponseToReport),
  };
};
