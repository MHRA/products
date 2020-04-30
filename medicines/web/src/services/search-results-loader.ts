import DataLoader from 'dataloader';
import { IDocument } from '../model/substance';
import { DocType } from './azure-search';
import { graphqlRequest } from './graphql';

interface IDocuments {
  count: number;
  edges: Array<{ node: IDocumentResponse }>;
}

interface ISearchPageResponse {
  documents: IDocuments;
}

interface IDocumentResponse {
  product: string;
  activeSubstances: string[];
  highlights: string[];
  created: string;
  docType: string;
  fileBytes: number;
  title: string;
  url: string;
}

const query = `
query($searchTerm: String, $first: Int, $after: String, $documentTypes: [String!]) {
  documents(search: $searchTerm, first: $first, after: $after, documentTypes: $documentTypes) {
    count: totalCount
    edges {
      cursor
      node {
        product: productName
        activeSubstances
        highlights
        created
        docType
        fileBytes: fileSizeInBytes
        title
        url
      }
    }
  }
}`;

interface ISearchPage {
  count: number;
  documents: IDocument[];
}

const convertResponseToSearchPage = ({
  documents: { count, edges },
}: ISearchPageResponse): ISearchPage => {
  return {
    count,
    documents: edges.map(convertDocumentResponseToDocument),
  };
};

const convertDocumentResponseToDocument = ({
  node: doc,
}: {
  node: IDocumentResponse;
}): IDocument => {
  return {
    activeSubstances: doc.activeSubstances,
    context: doc.highlights?.join(' … ') || '',
    created: doc.created,
    docType: doc.docType,
    fileSize: Math.ceil(doc.fileBytes / 1000).toLocaleString('en-GB'),
    name: doc.title,
    product: doc.product,
    url: doc.url,
  };
};

const getDocumentsForProduct = async ({
  searchTerm,
  page,
  pageSize,
  docTypes,
}: ISearchPageInfo) => {
  const variables = {
    searchTerm,
    first: pageSize,
    after: base64(calculatePageStartRecord(page, pageSize)),
    documentTypes: docTypes,
  };
  const { data } = await graphqlRequest<ISearchPageResponse, typeof variables>({
    query,
    variables,
  });

  return convertResponseToSearchPage(data);
};

interface ISearchPageInfo {
  searchTerm: string;
  page: number;
  pageSize: number;
  docTypes: DocType[];
}

const base64 = (skip: number): string => {
  return Buffer.from(skip.toString()).toString('base64');
};

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

export const searchResults = new DataLoader<ISearchPageInfo, ISearchPage>(
  async productPages => {
    return Promise.all(productPages.map(getDocumentsForProduct));
  },
);
