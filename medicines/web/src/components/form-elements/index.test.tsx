import renderer from 'react-test-renderer';
import React from 'react';
import { Button, Checkbox } from './index';

describe('Button', () => {
  it('should render', () => {
    const component = renderer
      .create(<Button type="submit" value="val" />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});

describe('Checkbox', () => {
  it('should render', () => {
    const changeHandler = () => {
      return null;
    };
    const component = renderer
      .create(
        <Checkbox
          value="val"
          name="agree"
          id="agree-checkbox"
          onChange={changeHandler}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
