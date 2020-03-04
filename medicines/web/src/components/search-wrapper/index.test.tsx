import { mount } from 'enzyme';
import React from 'react';
import SearchWrapper from './index';

describe(SearchWrapper, () => {
  it('should render', () => {
    const component = mount(
      <SearchWrapper initialSearchValue="initial value">
        <div>Child contents</div>
      </SearchWrapper>,
    );
    expect(component).toMatchSnapshot();
  });
});
