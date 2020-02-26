import { shallow } from 'enzyme';
import React from 'react';
import Pdf from './index';

describe(Pdf, () => {
  it('should render', () => {
    const component = shallow(<Pdf />);
    expect(component).toMatchSnapshot();
  });
});
