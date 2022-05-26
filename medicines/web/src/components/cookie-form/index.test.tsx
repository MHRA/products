import renderer from 'react-test-renderer';
import React from 'react';
import CookieForm from './index';

describe(CookieForm, () => {
  it('should render 🍪', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = renderer
      .create(<CookieForm storageAllowed setStorageAllowed={noop} />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });

  it('should render 🚫', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = renderer
      .create(<CookieForm storageAllowed={false} setStorageAllowed={noop} />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
