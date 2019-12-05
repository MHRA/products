import React from 'react';
import styled from 'styled-components';
import { black } from '../../styles/colors';

const StyledPagination = styled.nav`
  ul {
    display: flex;
    justify-content: space-between;
    list-style: none;
    margin: 0 auto;
    max-width: 80%;
    padding: 0;
  }

  .arrow a {
    color: ${black};
    text-decoration: none;
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
}) => {
  const paginationHref = (p: number) =>
    `/?search=${props.searchTerm}&page=${p}`;

  const pageCount = Math.floor(props.resultCount / props.pageSize) + 1;
  const { firstGroup, middleGroup, lastGroup } = getPaginationGroups(
    pageCount,
    props.currentPage,
  );

  const mapper = (p: number, i: number, array: number[]) => {
    const separator = i === array.length - 1 ? '' : <span>&ndash;</span>;

    if (p === props.currentPage) {
      return (
        <li key={p}>
          {p}
          {separator}
        </li>
      );
    }

    return (
      <li key={p}>
        <a href={paginationHref(p)}>{p}</a>
        {separator}
      </li>
    );
  };

  return (
    <StyledPagination>
      <ul>
        {props.currentPage !== 1 ? (
          <li className="arrow">
            <a href={paginationHref(props.currentPage - 1)}>Previous</a>
          </li>
        ) : (
          <li className="arrow" />
        )}
        <div className="pagination-number">
          {firstGroup.map(mapper)}
          {middleGroup.length > 0 ? <li>&hellip;</li> : ''}
          <div className="middle-group">{middleGroup.map(mapper)}</div>
          {lastGroup.length > 0 ? <li>&hellip;</li> : ''}
          {lastGroup.map(mapper)}
        </div>
        {props.currentPage !== pageCount ? (
          <li className="arrow">
            <a href={paginationHref(props.currentPage + 1)}>Next</a>
          </li>
        ) : (
          <li className="arrow" />
        )}
      </ul>
    </StyledPagination>
  );
};

export default Pagination;
