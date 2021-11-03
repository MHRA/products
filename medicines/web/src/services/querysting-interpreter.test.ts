import { DocType } from './azure-search';
import {
  docTypesFromQueryString,
  parsePage,
  queryStringFromTypes,
} from './querystring-interpreter';

describe(parsePage, () => {
  it('interprets blank as 1', () => {
    expect(parsePage('')).toBe(1);
  });

  it('interprets number string', () => {
    expect(parsePage('10')).toBe(10);
  });
});

describe(docTypesFromQueryString, () => {
  it('can separate by pipe', () => {
    expect(docTypesFromQueryString('Par|Pil')).toStrictEqual([
      DocType.Par,
      DocType.Pil,
    ]);
  });

  it('gives empty filters for empty string', () => {
    expect(docTypesFromQueryString('')).toStrictEqual([]);
  });
});

describe(queryStringFromTypes, () => {
  it('can join by pipe', () => {
    expect(queryStringFromTypes([DocType.Par, DocType.Pil])).toStrictEqual(
      'Par|Pil',
    );
  });

  it('gives empty string for empty filters', () => {
    expect(queryStringFromTypes([])).toStrictEqual('');
  });
});
