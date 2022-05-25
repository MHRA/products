import renderer from 'react-test-renderer';
import React from 'react';
import { RerouteType } from '../../../model/rerouteType';
import { DocType } from '../../../services/azure-search';
import SearchResults from './index';

const drugsMock = [
  {
    activeSubstances: ['substance 1', 'substance 2'],
    context: 'context',
    fileName: 'file name',
    fileSize: 'file size',
    fileUrl: 'file url',
    title: 'title',
    products: ['product 1', 'product 2'],
    matrices: ['matrix 1', 'matrix 2'],
    pbpkModels: ['model 1', 'model 2'],
    summary: 'summary',
    url: 'url',
    pregnancyTrimesters: ['first', 'second'],
    plNumbers: ['PL123451234', 'PL234562345'],
  },
  {
    activeSubstances: ['substance 3', 'substance 4'],
    context: 'context',
    fileName: 'file name',
    fileSize: 'file size',
    fileUrl: 'file url',
    title: 'title 2',
    products: ['product 3', 'product 4'],
    matrices: ['matrix 3', 'matrix 4'],
    pbpkModels: ['model 3', 'model 4'],
    summary: 'summary',
    url: 'url',
    pregnancyTrimesters: ['third'],
    plNumbers: [],
  },
];

const noFeedback = () => undefined;
const updateDocType = (d: DocType[]) => undefined;

describe(SearchResults, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <SearchResults
          reports={drugsMock}
          page={1}
          pageSize={20}
          resultCount={200}
          searchTerm={'Tea'}
          showingResultsForTerm={'Tea'}
          handlePageChange={noFeedback}
          isLoading={false}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });

  it('should render loading page', () => {
    const component = renderer
      .create(
        <SearchResults
          reports={drugsMock}
          page={1}
          pageSize={20}
          resultCount={200}
          searchTerm={'Tea'}
          showingResultsForTerm={'Tea'}
          handlePageChange={noFeedback}
          isLoading
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
