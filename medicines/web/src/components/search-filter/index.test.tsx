import { mount } from 'enzyme';
import React from 'react';
import { DocType } from '../../services/azure-search';
import SearchFilter from './index';

describe(SearchFilter, () => {
  it('should render search filter', () => {
    const component = mount(
      <SearchFilter 
        currentlyEnabledDocTypes={[]} 
        toggleDocType={(d: DocType)=>{}}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
