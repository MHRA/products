import { shallow } from 'enzyme';
import React from 'react';
import CookieForm from './index';

describe(CookieForm, () => {
  it('should render 🍪', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = shallow(
      <CookieForm storageAllowed setStorageAllowed={noop} />,
    );
    expect(component).toMatchSnapshot();
  });

  it('should render 🚫', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = shallow(
      <CookieForm storageAllowed={false} setStorageAllowed={noop} />,
    );
    expect(component).toMatchSnapshot();
  });
});
