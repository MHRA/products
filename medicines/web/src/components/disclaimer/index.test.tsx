import { shallow } from 'enzyme';
import React from 'react';
import Disclaimer from './index';

describe(Disclaimer, () => {
  it('should render', () => {
    const component = shallow(
      <Disclaimer onDisclaimerAgree={jest.fn()} searchTerm={'Perinola'} />,
    );
    expect(component).toMatchSnapshot();
  });
});
