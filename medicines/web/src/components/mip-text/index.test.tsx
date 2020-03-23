import { shallow } from 'enzyme';
import React from 'react';
import MipText from './index';

describe(MipText, () => {
  it('should render', () => {
    const component = shallow(<MipText />);
    expect(component).toMatchSnapshot();
  });
});
