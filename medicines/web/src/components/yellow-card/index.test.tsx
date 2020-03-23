import { shallow } from 'enzyme';
import React from 'react';
import YellowCard from './index';

describe(YellowCard, () => {
  it('should render', () => {
    const component = shallow(<YellowCard />);
    expect(component).toMatchSnapshot();
  });
});
