import { shallow } from 'enzyme';
import React from 'react';
import ProductList from './index';

describe(ProductList, () => {
  it('should render', () => {
    const component = shallow(
      <ProductList
        title={'Coffee'}
        products={[
          { name: 'Caffe Latte', count: 0 },
          { name: 'Americano', count: 1 },
          { name: 'Flat White', count: -1 },
          { name: 'Mocha', count: 9999999 },
          { name: 'Cafe Creme', count: -9999999 },
        ]}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
