import { shallow } from 'enzyme';
import React from 'react';
import { DocType } from '../../services/azure-search';
import Pagination from './pagination';

const page = 2;
const pageSize = 10;
const resultCount = 50;
const searchTerm = 'Caffeine';
const docTypes = [DocType.Pil, DocType.Par];

const dummyFunc = () => undefined;

describe(Pagination, () => {
  it('should render', () => {
    const component = shallow(
      <Pagination
        currentPage={page}
        pageSize={pageSize}
        resultCount={resultCount}
        searchTerm={searchTerm}
        enabledDocTypes={docTypes}
        handlePageChange={dummyFunc}
      />,
    );
    expect(component).toMatchSnapshot();
  });
});
