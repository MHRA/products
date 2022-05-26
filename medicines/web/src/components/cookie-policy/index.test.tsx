import renderer from 'react-test-renderer';
import React from 'react';
import CookieBanner from './index';

describe(CookieBanner, () => {
  it('should render 🍪', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = renderer
      .create(<CookieBanner storageAllowed setStorageAllowed={noop} />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
