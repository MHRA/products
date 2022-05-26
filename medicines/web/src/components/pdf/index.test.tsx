import renderer from 'react-test-renderer';
import React from 'react';
import Pdf from './index';

describe(Pdf, () => {
  it('should render', () => {
    const component = renderer.create(<Pdf />).toJSON();
    expect(component).toMatchSnapshot();
  });
});
