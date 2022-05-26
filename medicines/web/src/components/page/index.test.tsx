import renderer from 'react-test-renderer';
import React from 'react';
import App from './index';

describe(App, () => {
  it('should render', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = renderer
      .create(
        <App
          children={<></>}
          title={'MHRA'}
          metaTitle={'MHRA meta title'}
          storageAllowed={false}
          setStorageAllowed={noop}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
