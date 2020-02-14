import { DocType } from './azure-search';

export const parsePage = (page: string | string[]) => {
  let parsedPage = Number(page);
  if (!parsedPage || parsedPage < 1) {
    parsedPage = 1;
  }
  return parsedPage;
};

const formatDocTypeFilters = (s: string): DocType[] => {
  if (s.length <= 0) {
    return [];
  }

  return s.split(',').map(d => DocType[d as keyof typeof DocType]);
};

export const docTypesToFilter = (queryDocFilter: string | string[]) => {
  return typeof queryDocFilter === 'string' && queryDocFilter.length > 0
    ? formatDocTypeFilters(queryDocFilter)
    : null;
};
