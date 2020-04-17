import { shallow } from 'enzyme';
import React from 'react';
import ProductList from './index';

describe(ProductList, () => {
  it('should render', () => {
    const component = shallow(
      <ProductList
        title={'Coffee'}
        products={[
          { name: 'Caffe Latte', documentCount: 0 },
          { name: 'Americano', documentCount: 1 },
          { name: 'Flat White', documentCount: -1 },
          { name: 'Mocha', documentCount: 9999999 },
          { name: 'Cafe Creme', documentCount: -9999999 },
        ]}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
