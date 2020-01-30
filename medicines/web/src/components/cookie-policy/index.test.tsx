import { shallow } from 'enzyme';
import React from 'react';
import CookieBanner from './index';

describe(CookieBanner, () => {
  it('should render 🍪', () => {
    const component = shallow(<CookieBanner />);
    expect(component).toMatchSnapshot();
  });
});
