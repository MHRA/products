import { shallow } from 'enzyme';
import React from 'react';
import { Button } from './index';

describe('Button', () => {
  it('should render', () => {
    const component = shallow(<Button type="submit" value="val" />);
    expect(component).toMatchSnapshot();
  });
});
