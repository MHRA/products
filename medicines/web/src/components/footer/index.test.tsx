import renderer from 'react-test-renderer';
import React from 'react';
import Footer from './index';

describe(Footer, () => {
  it('should render', () => {
    const component = renderer.create(<Footer />).toJSON();
    expect(component).toMatchSnapshot();
  });
});
