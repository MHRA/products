import { shallow } from 'enzyme';
import React from 'react';
import App from './index';

describe(App, () => {
  it('should render', () => {
    // tslint:disable-next-line: no-empty
    const noop = () => {};
    const component = shallow(
      <App
        children={<></>}
        title={'MHRA'}
        storageAllowed
        setStorageAllowed={noop}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
