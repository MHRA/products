import { mount } from 'enzyme';
import React from 'react';
import { RerouteType } from '../../model/rerouteType';
import { DocType, TerritoryType } from '../../services/azure-search';
import SearchFilter from './index';

describe(SearchFilter, () => {
  it('should render search filter', () => {
    const updatePageFiltersFunction = (d: DocType[], t: TerritoryType[]) => {
      return null;
    };
    const component = mount(
      <SearchFilter
        currentlyEnabledDocTypes={[]}
        currentlyEnabledTerritoryTypes={[]}
        updatePageFilters={updatePageFiltersFunction}
        rerouteType={RerouteType.CheckboxSelected}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
