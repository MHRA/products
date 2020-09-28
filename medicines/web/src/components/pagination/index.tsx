import React from 'react';
import styled from 'styled-components';

import {
  mhra,
  mhra70,
  mhraBlue,
  mhraWhite,
  primaryColor,
} from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';
import { getPaginationGroups } from './pagination-groups';

const StyledPagination = styled.nav`
  div {
    display: flex;
    justify-content: space-between;
    list-style: none;
    margin: 20px auto 0px;
    max-width: 80%;
    padding: 0;
  }

  .pagination-number {
    display: flex;
    flex-grow: 1;
    justify-content: center;
    align-self: center;
    flex-wrap: nowrap;
    list-style: none;
    padding: 0 12px;
    margin: 0;
  }

  .pagination-number li {
    margin-right: 0.5rem;
  }

  li .link-text {
    padding-left: 0.5rem;
  }

  @media ${mobileBreakpoint} {
    font-size: 0.875rem;
  }

  .pagination .link-text {
    color: ${mhraBlue};
    text-decoration: underline;
  }

  .pagination .link-text:hover {
    cursor: pointer;
  }

  .arrow {
    color: ${mhraWhite};
    background-color: ${primaryColor};
    border-radius: 6px;
    text-decoration: none;
    appearance: none;
    border: 0;
    display: block;
    padding: 12px 15px;
    cursor: pointer;
    height: max-content;
  }

  .arrow:hover {
    background-color: ${mhra70};
  }

  @media ${mobileBreakpoint} {
    .arrow {
      padding: 7px 10px;
      border-radius: 4px;
    }
  }
`;

interface IPaginationProps {
  searchTerm: string;
  resultCount: number;
  pageSize: number;
  currentPage: number;
  handlePageChange: (num: number) => void;
}

const Pagination = (props: IPaginationProps) => {
  const pageCount = Math.floor(props.resultCount / props.pageSize) + 1;
  const { firstGroup, middleGroup, lastGroup } = getPaginationGroups(
    pageCount,
    props.currentPage,
  );

  const getPageChangeHandler = (num: number) => () =>
    props.handlePageChange(num);

  const createPaginationButton = (page: number, i: number, array: number[]) => {
    const separator = i === array.length - 1 ? '' : <span>&ndash;</span>;

    if (page === props.currentPage) {
      return (
        <li
          key={page}
          aria-label={`Current Page, Page ${page}`}
          aria-current="true"
        >
          {page}
          {separator}
        </li>
      );
    }

    return (
      <li
        key={page}
        onClick={getPageChangeHandler(page)}
        aria-label={`Goto Page ${page}`}
      >
        <span className="link-text">{page}</span>
        {separator}
      </li>
    );
  };

  return (
    <StyledPagination>
      <nav aria-label="Pagination Navigation">
        <div className="pagination">
          {props.currentPage !== 1 ? (
            <button
              className="arrow"
              onClick={getPageChangeHandler(props.currentPage - 1)}
              aria-label={`Goto previous page, Page ${props.currentPage - 1}`}
            >
              Previous
            </button>
          ) : (
            <></>
          )}
          <ul className="pagination-number">
            {firstGroup.map(createPaginationButton)}
            {middleGroup.length > 0 ? <li>&hellip;</li> : ''}
            {middleGroup.map(createPaginationButton)}
            {lastGroup.length > 0 ? <li>&hellip;</li> : ''}
            {lastGroup.map(createPaginationButton)}
          </ul>
          {props.currentPage !== pageCount ? (
            <button
              className="arrow"
              onClick={getPageChangeHandler(props.currentPage + 1)}
              aria-label={`Goto next page, Page ${props.currentPage + 1}`}
            >
              Next
            </button>
          ) : (
            <></>
          )}
        </div>
      </nav>
    </StyledPagination>
  );
};

export default Pagination;
