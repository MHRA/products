import { shallow } from 'enzyme';
import React from 'react';
import Header from './index';

describe(Header, () => {
  it('should render', () => {
    const component = shallow(<Header title={'MHRA'} />);
    expect(component).toMatchSnapshot();
  });
});
