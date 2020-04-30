import moment from 'moment';

import { IDocument } from '../model/substance';
import { ISearchResult } from '../services/azure-search';

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
