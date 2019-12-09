import Link from 'next/link';
import React from 'react';
import styled from 'styled-components';
import { mhraBlue, mhrawhite, primaryColor } from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';
import { getPaginationGroups } from './pagination-groups';

const StyledPagination = styled.nav`
  ul {
    display: flex;
    justify-content: space-between;
    list-style: none;
    margin: 40px auto 20px;
    max-width: 80%;
    padding: 0;
  }

  .pagination-number,
  .middle-group {
    display: flex;
    flex-wrap: nowrap;
  }

  .pagination-number li,
  .middle-group li {
    margin-right: 0.5rem;
  }

  li span {
    padding-left: 0.5rem;
  }

  @media ${mobileBreakpoint} {
    font-size: 0.875rem;
  }

  .pagination a {
    color: ${mhraBlue};
    text-decoration: underline;
  }

  .pagination a:hover {
    cursor: pointer;
  }

  .arrow a {
    color: ${mhrawhite};
    background-color: ${primaryColor};
    padding: 12px 15px;
    border-radius: 6px;
    text-decoration: none;
  }

  @media ${mobileBreakpoint} {
    .arrow a {
      padding: 7px 10px;
      border-radius: 4px;
    }
  }
`;

const Pagination = (props: {
  searchTerm: string;
  resultCount: number;
  pageSize: number;
  currentPage: number;
}) => {
  const pageCount = Math.floor(props.resultCount / props.pageSize) + 1;
  const { firstGroup, middleGroup, lastGroup } = getPaginationGroups(
    pageCount,
    props.currentPage,
  );

  const createPaginationButton = (page: number, i: number, array: number[]) => {
    const separator = i === array.length - 1 ? '' : <span>&ndash;</span>;

    if (page === props.currentPage) {
      return (
        <li key={page}>
          {page}
          {separator}
        </li>
      );
    }

    return (
      <li key={page}>
        <Link href={getSearchPage(page)}>
          <a>{page}</a>
        </Link>
        {separator}
      </li>
    );
  };

  const getSearchPage = (pageNo: number) => ({
    pathname: '',
    query: { search: props.searchTerm, page: pageNo },
  });

  return (
    <StyledPagination>
      <ul className="pagination">
        {props.currentPage !== 1 ? (
          <li className="arrow">
            <Link href={getSearchPage(props.currentPage - 1)}>
              <a>Previous</a>
            </Link>
          </li>
        ) : (
          <li className="arrow" />
        )}
        <div className="pagination-number">
          {firstGroup.map(createPaginationButton)}
          {middleGroup.length > 0 ? <li>&hellip;</li> : ''}
          <div className="middle-group">
            {middleGroup.map(createPaginationButton)}
          </div>
          {lastGroup.length > 0 ? <li>&hellip;</li> : ''}
          {lastGroup.map(createPaginationButton)}
        </div>
        {props.currentPage !== pageCount ? (
          <li className="arrow">
            <Link href={getSearchPage(props.currentPage + 1)}>
              <a>Next</a>
            </Link>
          </li>
        ) : (
          <li className="arrow" />
        )}
      </ul>
    </StyledPagination>
  );
};

export default Pagination;
