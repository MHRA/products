import renderer from 'react-test-renderer';
import React from 'react';
import Header from './index';

describe(Header, () => {
  it('should render', () => {
    const component = renderer.create(<Header title={'MHRA'} />).toJSON();
    expect(component).toMatchSnapshot();
  });
});
