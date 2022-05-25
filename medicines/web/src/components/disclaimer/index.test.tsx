import renderer from 'react-test-renderer';
import React from 'react';
import Disclaimer from './index';

describe(Disclaimer, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <Disclaimer onDisclaimerAgree={jest.fn()} searchTerm={'Perinola'} />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
