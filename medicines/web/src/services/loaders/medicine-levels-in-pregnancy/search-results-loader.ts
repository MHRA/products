import DataLoader from 'dataloader';
import { IBmgfReport, IBmgfReports } from '../../../model/document';
import { graphqlRequest } from '../../graphql';

interface IReportResponse {
  products: string[];
  activeSubstances: string[];
  title: string;
  highlights: string[];
  fileSizeInBytes: number;
  fileName: string;
  fileUrl: string;
  summary: string;
  matrices: string[];
  pbpkModels: string[];
  pregnancyTrimesters: string[];
  plNumbers: string[];
}

export interface IEdge {
  node: IReportResponse;
}

interface ISearchResponse {
  medicineLevelsInPregnancy: {
    reports: { count: number; edges: IEdge[] };
  };
}

export interface ISearchInfo {
  searchTerm: string;
  page: number;
  pageSize: number;
}

const query = `
query($searchTerm: String, $first: Int, $after: String) {
  medicineLevelsInPregnancy {
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
}`;

export const reportsLoader = new DataLoader<ISearchInfo, IBmgfReports>(
  async (searchTerms) => {
    return Promise.all(searchTerms.map(getReportsForSearch));
  },
);

const getReportsForSearch = async ({
  searchTerm,
  page,
  pageSize,
}: ISearchInfo): Promise<IBmgfReports> => {
  const variables = {
    searchTerm,
    first: pageSize,
    after: makeCursor(page, pageSize),
  };
  const { data } = await graphqlRequest<ISearchResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToReports(data);
};

const convertResponseToReports = ({
  medicineLevelsInPregnancy: {
    reports: { count, edges },
  },
}: ISearchResponse): IBmgfReports => {
  return {
    count,
    reports: edges.map(convertReportResponseToReport),
  };
};

export const convertReportResponseToReport = ({
  node: report,
}: {
  node: IReportResponse;
}): IBmgfReport => {
  return {
    activeSubstances: report.activeSubstances,
    context: report.highlights?.join(' … ') || '',
    fileName: report.fileName,
    fileUrl: report.fileUrl,
    products: report.products,
    summary: report.summary,
    pbpkModels: report.pbpkModels,
    matrices: report.matrices,
    title: report.title,
    fileSize: Math.ceil(report.fileSizeInBytes / 1000).toLocaleString('en-GB'),
    url: `/medicine-levels-in-pregnancy/reports/${report.title}`,
    pregnancyTrimesters: report.pregnancyTrimesters,
    plNumbers: report.plNumbers,
  };
};

export const makeCursor = (page: number, pageSize: number): string => {
  const skip = calculatePageStartRecord(page, pageSize);

  return Buffer.from((skip - 1).toString()).toString('base64');
};

export const calculatePageStartRecord = (
  page: number,
  pageSize: number,
): number => pageSize * (page - 1);
