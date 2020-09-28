import {
  convertResults,
  convertBmgfResults,
  mapSubstancesIndex,
  mapProductsIndex,
} from './azure-results-converter';

import { IDocument, IBmgfReport } from '../model/document';
import { IFacet } from '../model/facet';
import {
  IFacetResult,
  ISearchResult,
  DocType,
  IBmgfSearchResult,
} from '../services/azure-search';

describe(convertResults, () => {
  it('properly converts results', () => {
    const searchResult: ISearchResult = {
      '@search.highlights': { content: ['highlight 1', 'highlight 2'] },
      '@search.score': 9.8,
      author: 'author',
      created: '2020-01-05',
      doc_type: DocType.Spc,
      file_name: 'file name',
      keywords: 'keyword1 keyword2',
      metadata_storage_name: 'storage name',
      metadata_storage_path: 'storage/path',
      metadata_storage_size: 1234.3,
      product_name: 'product name',
      release_state: 'released',
      substance_name: ['substance 1', 'substance 2'],
      suggestions: ['suggestion 1', 'suggestion 2'],
      title: 'title',
    };
    const convertedResult = convertResults(searchResult);
    expect(convertedResult.activeSubstances.length).toBe(2);
    expect(convertedResult.activeSubstances[0]).toBe('substance 1');
    expect(convertedResult.product).toBe('product name');
    expect(convertedResult.context).toBe('highlight 1 … highlight 2');
    expect(convertedResult.docType).toBe('Spc');
    expect(convertedResult.fileSize).toBe('2');
    expect(convertedResult.created).toBe('5 January 2020');
    expect(convertedResult.name).toBe('title');
    expect(convertedResult.url).toBe('storage/path');
  });
});

describe(convertBmgfResults, () => {
  it('properly converts results', () => {
    const searchResult: IBmgfSearchResult = {
      '@search.highlights': { content: ['highlight 1', 'highlight 2'] },
      '@search.score': 9.8,
      file_name: 'file name',
      metadata_storage_name: 'storage name',
      metadata_storage_path: 'storage/path',
      metadata_storage_size: 1234.3,
      products: ['product 1', 'product 2'],
      active_substances: ['substance 1', 'substance 2'],
      summary: 'Summary',
      pbpk_models: ['model 1', 'model 2'],
      matrices: ['matrix 1', 'matrix 2'],
      pl_numbers: ['PL123451234', 'PL234562345'],
      report_name: 'report name',
      pregnancy_trimesters: ['first', 'second'],
    };
    const convertedResult = convertBmgfResults(searchResult);
    expect(convertedResult.activeSubstances.length).toBe(2);
    expect(convertedResult.activeSubstances[0]).toBe('substance 1');
    expect(convertedResult.context).toBe('highlight 1 … highlight 2');
    expect(convertedResult.fileName).toBe('file name');
    expect(convertedResult.fileSize).toBe('2');
    expect(convertedResult.products.length).toBe(2);
    expect(convertedResult.products[0]).toBe('product 1');
    expect(convertedResult.matrices.length).toBe(2);
    expect(convertedResult.matrices[0]).toBe('matrix 1');
    expect(convertedResult.pbpkModels.length).toBe(2);
    expect(convertedResult.pbpkModels[0]).toBe('model 1');
    expect(convertedResult.summary).toBe('Summary');
    expect(convertedResult.url).toBe(
      '/medicine-levels-in-pregnancy/reports/report name',
    );
    expect(convertedResult.pregnancyTrimesters.length).toBe(2);
    expect(convertedResult.pregnancyTrimesters[0]).toBe('first');
    expect(convertedResult.plNumbers.length).toBe(2);
    expect(convertedResult.plNumbers[0]).toBe('PL123451234');
  });
});

describe(mapSubstancesIndex, () => {
  it('properly converts results', () => {
    const facetResult = {
      facets: [
        { count: 20, value: 'A' },
        { count: 12, value: 'A, ACTIVE SUBSTANCE2' },
        { count: 12, value: 'A, ACTIVE SUBSTANCE' },
        { count: 8, value: 'A, ACTIVE SUBSTANCE, PRODUCT1' },
        { count: 2, value: 'A, ACTIVE  SUBSTANCE, PRODUCT1' },
        { count: 2, value: 'A, ACTIVE SUBSTANCE, PRODUCT2' },
        { count: 6, value: 'A, ACTIVE SUBSTANCE, PRODUCT3' },
        { count: 1, value: 'A, ACTIVE SUBSTANCE2, PRODUCT1' },
        { count: 5, value: 'A, ACTIVE SUBSTANCE2, PRODUCT4' },
        { count: 6, value: 'A, ACTIVE SUBSTANCE2, PRODUCT5' },
        { count: 8, value: 'O, OTHER ACTIVE SUBSTANCE' },
        { count: 3, value: 'O, OTHER ACTIVE SUBSTANCE, PRODUCT5' },
        { count: 5, value: 'O, OTHER ACTIVE SUBSTANCE, PRODUCT6' },
      ],
    };
    const results = mapSubstancesIndex(['A', facetResult]);
    expect(results.length).toBe(2);
    expect(results[0].name).toBe('ACTIVE SUBSTANCE');
    expect(results[0].count).toBe(12);
    expect(results[1].name).toBe('ACTIVE SUBSTANCE2');
    expect(results[1].count).toBe(12);
  });
});

describe(mapProductsIndex, () => {
  it('properly converts results', () => {
    const facetResult = {
      facets: [
        { count: 20, value: 'A' },
        { count: 10, value: 'A, ACTIVE SUBSTANCE' },
        { count: 6, value: 'A, ACTIVE SUBSTANCE, PRODUCT3' },
        { count: 8, value: 'A, ACTIVE SUBSTANCE, PRODUCT1' },
        { count: 2, value: 'A, ACTIVE SUBSTANCE, PRODUCT2' },
        { count: 12, value: 'A, ACTIVE SUBSTANCE2' },
        { count: 1, value: 'A, ACTIVE SUBSTANCE2, PRODUCT1' },
        { count: 5, value: 'A, ACTIVE SUBSTANCE2, PRODUCT4' },
        { count: 6, value: 'A, ACTIVE SUBSTANCE2, PRODUCT5' },
        { count: 8, value: 'O, OTHER ACTIVE SUBSTANCE' },
        { count: 3, value: 'O, OTHER ACTIVE SUBSTANCE, PRODUCT3' },
        { count: 5, value: 'O, OTHER ACTIVE SUBSTANCE, PRODUCT6' },
      ],
    };
    const results = mapProductsIndex(['A, ACTIVE SUBSTANCE', facetResult]);
    expect(results.length).toBe(3);
    expect(results[0].name).toBe('PRODUCT1');
    expect(results[0].count).toBe(8);
    expect(results[1].name).toBe('PRODUCT2');
    expect(results[1].count).toBe(2);
    expect(results[2].name).toBe('PRODUCT3');
    expect(results[2].count).toBe(6);
  });
});
