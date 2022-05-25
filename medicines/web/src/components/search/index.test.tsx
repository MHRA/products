import renderer from 'react-test-renderer';
import React from 'react';
import Search from './index';

const onSearchBlurMock = jest.fn();
const onSearchChangeMock = jest.fn();
const onSearchSubmitMock = jest.fn();
const searchMock = 'drug';

describe(Search, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <Search
          onSearchBlur={onSearchBlurMock}
          onSearchChange={onSearchChangeMock}
          onSearchSubmit={onSearchSubmitMock}
          search={searchMock}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
