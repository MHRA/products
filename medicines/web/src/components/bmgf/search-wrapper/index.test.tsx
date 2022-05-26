/**
 * @jest-environment jsdom
 */
// Added to run with jsdom test-environment so mount function doesn't
// throw global document not loaded error: https://stackoverflow.com/a/45460795/5467902

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
