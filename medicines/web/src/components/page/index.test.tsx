import { shallow } from 'enzyme';
import React from 'react';
import App from './index';

describe(App, () => {
  it('should render', () => {
    const component = shallow(<App children={<></>} title={'MHRA'} />);
    expect(component).toMatchSnapshot();
  });
});
