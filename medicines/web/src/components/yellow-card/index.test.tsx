import renderer from 'react-test-renderer';
import React from 'react';
import YellowCard from './index';

describe(YellowCard, () => {
  it('should render', () => {
    const component = renderer.create(<YellowCard />).toJSON();
    expect(component).toMatchSnapshot();
  });
});
