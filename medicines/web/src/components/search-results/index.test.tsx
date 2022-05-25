import renderer from 'react-test-renderer';
import React from 'react';
import { RerouteType } from '../../model/rerouteType';
import { DocType } from '../../services/azure-search';
import SearchResults from './index';

const drugsMock = [
  {
    activeSubstances: ['tea', 'coffee'],
    context: 'string',
    created: 'string',
    docType: 'string',
    fileSize: 'string',
    name: 'string',
    product: 'string',
    url: 'string',
  },
  {
    activeSubstances: ['tea', 'coffee'],
    context: 'string',
    created: 'string',
    docType: 'string',
    fileSize: 'string',
    name: 'string',
    product: 'string',
    url: 'string',
  },
];

const noFeedback = () => undefined;
const updateDocType = (d: DocType[]) => undefined;

describe(SearchResults, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <SearchResults
          drugs={drugsMock}
          page={1}
          pageSize={20}
          resultCount={200}
          searchTerm={'Tea'}
          showingResultsForTerm={'Tea'}
          disclaimerAgree
          docTypes={[]}
          territoryTypes={[]}
          updatePageFilters={updateDocType}
          rerouteType={RerouteType.CheckboxSelected}
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
          drugs={drugsMock}
          page={1}
          pageSize={20}
          resultCount={200}
          searchTerm={'Tea'}
          showingResultsForTerm={'Tea'}
          disclaimerAgree
          docTypes={[]}
          territoryTypes={[]}
          rerouteType={RerouteType.Other}
          updatePageFilters={noFeedback}
          handlePageChange={noFeedback}
          isLoading
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });

  it('should render error message', () => {
    const component = renderer
      .create(
        <SearchResults
          drugs={drugsMock}
          page={1}
          pageSize={20}
          resultCount={200}
          searchTerm={'Tea'}
          showingResultsForTerm={'Tea'}
          disclaimerAgree
          docTypes={[]}
          territoryTypes={[]}
          rerouteType={RerouteType.Other}
          updatePageFilters={noFeedback}
          handlePageChange={noFeedback}
          isLoading
          errorFetchingResults
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
