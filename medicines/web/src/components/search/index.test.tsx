import { shallow } from 'enzyme';
import React from 'react';
import Search from './index';

const onSearchBlurMock = jest.fn();
const onSearchChangeMock = jest.fn();
const onSearchSubmitMock = jest.fn();
const searchMock = 'drug';

describe(Search, () => {
  it('should render', () => {
    const component = shallow(
      <Search
        onSearchBlur={onSearchBlurMock}
        onSearchChange={onSearchChangeMock}
        onSearchSubmit={onSearchSubmitMock}
        search={searchMock}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
