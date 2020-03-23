import { DocType } from './azure-search';

export const parsePage = (page: string | string[]) => {
  let parsedPage = Number(page);
  if (!parsedPage || parsedPage < 1) {
    parsedPage = 1;
  }
  return parsedPage;
};

const docTypeQueryStringSeparator = '|';

const formatDocTypeFilters = (s: string): DocType[] => {
  if (s.length <= 0) {
    return [];
  }

  return s
    .split(docTypeQueryStringSeparator)
    .map(d => DocType[d as keyof typeof DocType]);
};

export const queryStringFromDocTypes = (enabledDocTypes: DocType[]): string =>
  enabledDocTypes.length > 0
    ? enabledDocTypes.join(docTypeQueryStringSeparator)
    : '';

export const docTypesFromQueryString = (
  queryDocFilter: string | string[],
): DocType[] => {
  return typeof queryDocFilter === 'string' && queryDocFilter.length > 0
    ? formatDocTypeFilters(queryDocFilter)
    : [];
};

export const parseDisclaimerAgree = (
  queryDisclaimerAgree: string | string[],
): boolean => {
  return queryDisclaimerAgree === 'agree';
};
