import React from 'react';
import styled from 'styled-components';
import { black, mhraBlue } from '../../styles/colors';
import { mobileBreakpoint } from '../../styles/dimensions';

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
`;

const getPaginationGroups = (pageCount: number, currentPage: number) => {
  let firstGroup: number[] = [1];
  let middleGroup: number[] = [];
  let lastGroup: number[] = [pageCount];

  if (pageCount < 6) {
    return {
      firstGroup: Array(pageCount)
        .fill(1)
        .map((_, i) => i + 1),
      middleGroup,
      lastGroup: [],
    };
  }

  switch (currentPage) {
    case 1:
    case 2:
    case 3:
      firstGroup = Array(currentPage + 1)
        .fill(1)
        .map((_, i) => i + 1);
      break;
    case pageCount - 2:
    case pageCount - 1:
    case pageCount:
      lastGroup = Array(pageCount - currentPage + 2)
        .fill(1)
        .map((_, i) => currentPage - 1 + i);
      break;
    default:
      middleGroup = [currentPage - 1, currentPage, currentPage + 1];
      break;
  }
  return {
    firstGroup,
    middleGroup,
    lastGroup,
  };
};

const Pagination = (props: {
  searchTerm: string;
  resultCount: number;
  pageSize: number;
  currentPage: number;
  callback: (pageNo: number) => void;
}) => {
  const paginationHref = (p: number) =>
    `/?search=${props.searchTerm}&page=${p}`;

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
        {/* tslint:disable-next-line:jsx-no-lambda */}
        <a onClick={_ => props.callback(page)}>{page}</a>
        {separator}
      </li>
    );
  };

  return (
    <StyledPagination>
      <ul className="pagination">
        {props.currentPage !== 1 ? (
          <li className="arrow">
            {/* tslint:disable-next-line:jsx-no-lambda */}
            <a onClick={_ => props.callback(props.currentPage - 1)}>Previous</a>
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
            {/* tslint:disable-next-line:jsx-no-lambda */}
            <a onClick={_ => props.callback(props.currentPage + 1)}>Next</a>
          </li>
        ) : (
          <li className="arrow" />
        )}
      </ul>
    </StyledPagination>
  );
};

export default Pagination;
