import { shallow } from 'enzyme';
import React from 'react';
import Mip from './index';

jest.mock('next/router', () => ({
  useRouter: () => ({
    query: {
      searchTerm: 'drug',
      page: 1,
      substance: 'whisky',
      disclaimer: true,
    },
  }),
}));

describe(Mip, () => {
  it('should render', () => {
    const component = shallow(<Mip />);
    expect(component).toMatchSnapshot();
  });
});
