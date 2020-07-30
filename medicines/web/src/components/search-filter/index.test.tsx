import { mount } from 'enzyme';
import React from 'react';
import { RerouteType } from '../../model/rerouteType';
import { DocType } from '../../services/azure-search';
import SearchFilter from './index';

describe(SearchFilter, () => {
  it('should render search filter', () => {
    const updateDocTypesFunction = (d: DocType[]) => {
      return null;
    };
    const component = mount(
      <SearchFilter
        currentlyEnabledDocTypes={[]}
        updateDocTypes={updateDocTypesFunction}
        rerouteType={RerouteType.CheckboxSelected}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
