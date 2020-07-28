import { shallow } from 'enzyme';
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
    const component = shallow(
      <SearchResults
        drugs={drugsMock}
        page={1}
        pageSize={20}
        resultCount={200}
        searchTerm={'Tea'}
        showingResultsForTerm={'Tea'}
        disclaimerAgree
        docTypes={[]}
        updateDocTypes={updateDocType}
        rerouteType={RerouteType.CheckboxSelected}
        handlePageChange={noFeedback}
        isLoading={false}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
