import { shallow } from 'enzyme';
import React from 'react';
import Disclaimer from './index';

const fn = jest.fn();

describe(Disclaimer, () => {
  it('should render ⚠️', () => {
    const component = shallow(
      <Disclaimer onDisclaimerAgree={fn} searchTerm={'Perinola'} />,
    );
    expect(component).toMatchSnapshot();
  });
});
