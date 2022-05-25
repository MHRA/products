import renderer from 'react-test-renderer';
import React from 'react';
import Pagination from '.';

const page = 2;
const pageSize = 10;
const resultCount = 50;
const searchTerm = 'Caffeine';

const dummyFunc = () => undefined;

describe(Pagination, () => {
  it('should render', () => {
    const component = renderer
      .create(
        <Pagination
          currentPage={page}
          pageSize={pageSize}
          resultCount={resultCount}
          searchTerm={searchTerm}
          handlePageChange={dummyFunc}
        />,
      )
      .toJSON();
    expect(component).toMatchSnapshot();
  });
});
