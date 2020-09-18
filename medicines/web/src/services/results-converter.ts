import moment from 'moment';

import { IDocument, IBmgfDocument } from '../model/substance';
import { ISearchResult, IBmgfSearchResult } from '../services/azure-search';

const sanitizeTitle = (title: string | null): string => {
  let name: string;
  if (!title) return 'Unknown';

  try {
    name = decodeURIComponent(title);
  } catch {
    name = title;
  }
  return name;
};

export const convertResults = (doc: ISearchResult): IDocument => {
  return {
    activeSubstances: doc.substance_name,
    product: doc.product_name,
    context: doc['@search.highlights']?.content.join(' … ') || '',
    docType: doc.doc_type?.toString().substr(0, 3) || '',
    fileSize: Math.ceil(
      (doc.metadata_storage_size ? doc.metadata_storage_size : 0) / 1000,
    ).toLocaleString('en-GB'),
    created: doc.created
      ? moment(doc.created).format('D MMMM YYYY')
      : 'Unknown',
    name: sanitizeTitle(doc.title),
    url: doc.metadata_storage_path,
  };
};

export const convertBmgfResults = (doc: IBmgfSearchResult): IBmgfDocument => {
  return {
    activeSubstances: doc.active_substances,
    context: doc['@search.highlights']?.content.join(' … ') || '',
    fileName: doc.file_name || '',
    fileUrl: doc.metadata_storage_path,
    products: doc.products,
    summary: doc.summary,
    pbpkModels: doc.pbpk_models,
    matrices: doc.matrices,
    title: sanitizeTitle(doc.report_name),
    fileSize: Math.ceil(
      (doc.metadata_storage_size ? doc.metadata_storage_size : 0) / 1000,
    ).toLocaleString('en-GB'),
    url: doc.metadata_storage_path.replace('.pdf', '.html'),
  };
};
