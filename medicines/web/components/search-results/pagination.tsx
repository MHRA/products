import React from 'react';

const Pagination = (props: {
  searchTerm: string;
  resultCount: number;
  pageSize: number;
  currentPage: number;
}) => {
  const paginationHref = (p: number) =>
    `/?search=${props.searchTerm}&page=${p}`;

  const pageCount = Math.floor(props.resultCount / props.pageSize) + 1;
  let firstGroup: number[] = [1];
  let middleGroup: number[] = [];
  let lastGroup: number[] = [pageCount];

  if (props.currentPage === 1) {
    firstGroup = [1, 2];
  } else if (props.currentPage === 2) {
    firstGroup = [1, 2, 3];
  } else if (props.currentPage === 3) {
    firstGroup = [1, 2, 3, 4];
  } else if (props.currentPage === pageCount - 2) {
    lastGroup = [pageCount - 3, pageCount - 2, pageCount - 1, pageCount];
  } else if (props.currentPage === pageCount - 1) {
    lastGroup = [pageCount - 2, pageCount - 1, pageCount];
  } else if (props.currentPage === pageCount) {
    lastGroup = [pageCount - 1, pageCount];
  } else {
    middleGroup = [
      props.currentPage - 1,
      props.currentPage,
      props.currentPage + 1,
    ];
  }

  const mapper = (p: number) => {
    if (p === props.currentPage) {
      return <li key={p}>{p}</li>;
    }

    return (
      <li key={p}>
        <a href={paginationHref(p)}>{p}</a>
      </li>
    );
  };

  return (
    <nav>
      <ul>
        {firstGroup.map(mapper)}
        {middleGroup.length > 0 ? <li>...</li> : ''}
        {middleGroup.map(mapper)}
        <li>...</li>
        {lastGroup.map(mapper)}
      </ul>
    </nav>
  );
};

export default Pagination;
