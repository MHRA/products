import { makeCursor } from './search-results-loader';

describe(makeCursor, () => {
  it.each([2, 4, 5, 10, 15, 20])('offsets first result by -1', (pageSize) => {
    const cursor = makeCursor(1, pageSize);
    expect(cursor).toBe('LTE=');
  });

  it.each([
    [2, 10, 'OQ=='],
    [3, 5, 'OQ=='],
    [2, 2, 'MQ=='],
    [3, 1, 'MQ=='],
    [2, 3, 'Mg=='],
    [4, 1, 'Mg=='],
  ])(
    'page %i for pageSize %i results starts after %s',
    (page, pageSize, expectedCursor) => {
      const cursor = makeCursor(page, pageSize);
      expect(cursor).toBe(expectedCursor);
    },
  );
});
