import renderer from 'react-test-renderer';
import React from 'react';
import { RerouteType } from '../../model/rerouteType';
import { DocType, TerritoryType } from '../../services/azure-search';
import SearchFilter from './index';

describe(SearchFilter, () => {
  it('should render search filter', () => {
    const updatePageFiltersFunction = (d: DocType[], t: TerritoryType[]) => {
      return null;
    };
    const component = renderer
      .create(
        <SearchFilter
          currentlyEnabledDocTypes={[]}
          currentlyEnabledTerritoryTypes={[]}
          updatePageFilters={updatePageFiltersFunction}
          rerouteType={RerouteType.CheckboxSelected}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
