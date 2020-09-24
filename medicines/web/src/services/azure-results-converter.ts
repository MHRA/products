import moment from 'moment';

import { IDocument, IBmgfReport } from '../model/document';
import { IFacet } from '../model/facet';
import {
  IFacetResult,
  ISearchResult,
  IBmgfSearchResult,
} from '../services/azure-search';

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

export const convertBmgfResults = (doc: IBmgfSearchResult): IBmgfReport => {
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
    url: `/medicine-levels-in-pregnancy/reports/${doc.report_name}`,
    plNumbers: doc.pl_numbers,
    pregnancyTrimesters: doc.pregnancy_trimesters,
  };
};

export const mapSubstancesIndex = ([letter, facetResult]: [
  string,
  IFacetResult,
]): IFacet[] => {
  const indexResults: { [id: string]: IFacet } = {};
  facetResult.facets
    .filter((x) => x.value.startsWith(letter))
    .forEach((f) => {
      const [substance, product] = f.value
        .replace(/\s+/g, ' ')
        .split(', ', 3)
        .slice(1);

      if (substance && !product) {
        const currentSubstanceCount = indexResults[substance]?.count ?? 0;
        indexResults[substance] = {
          name: substance,
          count: currentSubstanceCount + f.count,
        };
      }
    });
  return Object.values(indexResults).sort((a, b) => (a.name < b.name ? -1 : 1));
};

export const mapProductsIndex = ([letterAndSubstance, facetResult]: [
  string,
  IFacetResult,
]): IFacet[] => {
  const indexResults: { [id: string]: IFacet } = {};
  const requiredFacetPrefix = `${letterAndSubstance},`;
  facetResult.facets.forEach((f) => {
    const sanitizedFacet = f.value.replace(/\s+/g, ' ');
    if (!sanitizedFacet.startsWith(requiredFacetPrefix)) {
      return;
    }

    const [product] = sanitizedFacet.split(', ', 3).slice(2);

    if (product) {
      const currentProductCount = indexResults[product]?.count ?? 0;
      indexResults[product] = {
        name: product,
        count: currentProductCount + f.count,
      };
    }
  });
  return Object.values(indexResults).sort((a, b) => (a.name < b.name ? -1 : 1));
};
