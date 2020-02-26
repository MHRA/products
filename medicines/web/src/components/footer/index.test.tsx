import { shallow } from 'enzyme';
import React from 'react';
import Footer from './index';

describe(Footer, () => {
  it('should render', () => {
    const component = shallow(<Footer />);
    expect(component).toMatchSnapshot();
  });
});
