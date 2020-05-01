import { makeCursor } from './search-results-loader';

describe(makeCursor, () => {
  it('gets first result', () => {
    const cursor = makeCursor(1, 10);
    expect(cursor).toBe('LTE=');
  });

  it('gets second page result', () => {
    const cursor = makeCursor(2, 10);
    expect(cursor).toBe('OQ==');
  });
});
