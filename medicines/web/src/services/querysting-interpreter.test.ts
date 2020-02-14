import { DocType } from './azure-search';
import { docTypesToFilter, parsePage } from './querystring-interpreter';

describe(parsePage, () => {
  it('interprets blank as 1', () => {
    expect(parsePage('')).toBe(1);
  });

  it('interprets number string', () => {
    expect(parsePage('10')).toBe(10);
  });
});

describe(docTypesToFilter, () => {
  it('can separate by comma', () => {
    expect(docTypesToFilter('Par,Pil')).toStrictEqual([
      DocType.Par,
      DocType.Pil,
    ]);
  });
});
