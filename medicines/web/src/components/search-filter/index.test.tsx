import { mount } from 'enzyme';
import React from 'react';
import { DocType } from '../../services/azure-search';
import SearchFilter from './index';

describe(SearchFilter, () => {
  it('should render search filter', () => {
    const toggleFunction = (d: DocType) => {
      return null;
    };
    const component = mount(
      <SearchFilter
        currentlyEnabledDocTypes={[]}
        toggleDocType={toggleFunction}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
