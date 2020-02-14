import { shallow } from 'enzyme';
import React from 'react';
import CookieBanner from './index';

describe(CookieBanner, () => {
  it('should render 🍪', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = shallow(
      <CookieBanner storageAllowed setStorageAllowed={noop} />,
    );
    expect(component).toMatchSnapshot();
  });
});
