import { shallow } from 'enzyme';
import React from 'react';
import { Button, Checkbox } from './index';

describe('Button', () => {
  it('should render', () => {
    const component = shallow(<Button type="submit" value="val" />);
    expect(component).toMatchSnapshot();
  });
});

describe('Checkbox', () => {
  it('should render', () => {
    const changeHandler = () => {
      return null;
    };
    const component = shallow(
      <Checkbox
        value="val"
        name="agree"
        id="agree-checkbox"
        onChange={changeHandler}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
