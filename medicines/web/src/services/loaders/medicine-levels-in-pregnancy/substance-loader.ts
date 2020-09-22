import DataLoader from 'dataloader';
import { IBmgfReport } from '../../../model/document';
import {
  IEdge,
  convertReportResponseToReport,
  calculatePageStartRecord,
} from './search-results-loader';
import { graphqlRequest } from '../../graphql';

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

export const substanceLoader = new DataLoader<ISubstanceInfo, ISubstance>(
  async (substances) => {
    return Promise.all(substances.map(getReportsForSubstance));
  },
);

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
