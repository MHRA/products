interface IPaginationGroups {
  firstGroup: number[];
  middleGroup: number[];
  lastGroup: number[];
}

export const getPaginationGroups = (
  pageCount: number,
  currentPage: number,
): IPaginationGroups => {
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
