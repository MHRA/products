import React from 'react';
import styled from 'styled-components';
import {
  mhra70,
  mhraBlue,
  mhraWhite,
  primaryColor,
} from '../../../styles/colors';
import { mobileBreakpoint } from '../../../styles/dimensions';
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

  .arrow .link-text {
    color: ${mhraWhite};
    background-color: ${primaryColor};
    padding: 12px 15px;
    border-radius: 6px;
    text-decoration: none;
  }

  .arrow .link-text:hover {
    background-color: ${mhra70};
  }

  @media ${mobileBreakpoint} {
    .arrow .link-text {
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
      <nav role="navigation" aria-label="Pagination Navigation">
        <ul className="pagination">
          {props.currentPage !== 1 ? (
            <li
              className="arrow"
              onClick={getPageChangeHandler(props.currentPage - 1)}
              aria-label={`Goto previous page, Page ${props.currentPage - 1}`}
            >
              <span className="link-text">Previous</span>
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
            <li
              className="arrow"
              onClick={getPageChangeHandler(props.currentPage + 1)}
              aria-label={`Goto next page, Page ${props.currentPage + 1}`}
            >
              <span className="link-text">Next</span>
            </li>
          ) : (
            <li className="arrow" />
          )}
        </ul>
      </nav>
    </StyledPagination>
  );
};

export default Pagination;
