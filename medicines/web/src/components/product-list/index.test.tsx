import renderer from 'react-test-renderer';
import React from 'react';
import ProductList from './index';

describe(ProductList, () => {
  it('should render', () => {
    const component = renderer
      .create(
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
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
  it('should render error message', () => {
    const component = renderer
      .create(
        <ProductList
          title={'Coffee'}
          products={[
            { name: 'Caffe Latte', count: 0 },
            { name: 'Americano', count: 1 },
            { name: 'Flat White', count: -1 },
            { name: 'Mocha', count: 9999999 },
            { name: 'Cafe Creme', count: -9999999 },
          ]}
          errorFetchingResults
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
  it('should render loading message', () => {
    const component = renderer
      .create(<ProductList title={'Coffee'} products={[]} isLoading />)
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
