import renderer from 'react-test-renderer';
import React from 'react';
import SearchWrapper from './index';

describe(SearchWrapper, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <SearchWrapper initialSearchValue="initial value">
          <div>Child contents</div>
        </SearchWrapper>,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
