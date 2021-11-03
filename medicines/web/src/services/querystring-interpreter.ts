import { DocType, TerritoryType } from './azure-search';

export const parsePage = (page: string | string[]) => {
  let parsedPage = Number(page);
  if (!parsedPage || parsedPage < 1) {
    parsedPage = 1;
  }
  return parsedPage;
};

const typeQueryStringSeparator = '|';

const formatDocTypeFilters = (s: string): DocType[] => {
  if (s.length <= 0) {
    return [];
  }

  return s
    .split(typeQueryStringSeparator)
    .map((d) => DocType[d as keyof typeof DocType]);
};

export const docTypesFromQueryString = (
  queryDocFilter: string | string[],
): DocType[] => {
  return typeof queryDocFilter === 'string' && queryDocFilter.length > 0
    ? formatDocTypeFilters(queryDocFilter)
    : [];
};

const formatTerritoryTypeFilters = (s: string): TerritoryType[] => {
  if (s.length <= 0) {
    return [];
  }

  return s
    .split(typeQueryStringSeparator)
    .map((d) => TerritoryType[d as keyof typeof TerritoryType]);
};

export const territoryTypesFromQueryString = (
  queryTerritoryFilter: string | string[],
): TerritoryType[] => {
  return typeof queryTerritoryFilter === 'string' &&
    queryTerritoryFilter.length > 0
    ? formatTerritoryTypeFilters(queryTerritoryFilter)
    : [];
};

export const queryStringFromTypes = (
  enabledTypes: DocType[] | TerritoryType[],
): string =>
  enabledTypes.length > 0 ? enabledTypes.join(typeQueryStringSeparator) : '';

export const parseDisclaimerAgree = (
  queryDisclaimerAgree: string | string[],
): boolean => {
  return queryDisclaimerAgree === 'agree';
};
