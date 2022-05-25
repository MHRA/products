import renderer from 'react-test-renderer';
import React from 'react';
import MipText from './index';

describe(MipText, () => {
  it('should render', () => {
    const component = renderer.create(<MipText />).toJSON();
    expect(component).toMatchSnapshot();
  });
});
