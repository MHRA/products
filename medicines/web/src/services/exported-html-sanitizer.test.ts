import { getCleanedHtml } from './exported-html-sanitizer';

describe(getCleanedHtml, () => {
  it('interprets blank as 1', () => {
    getCleanedHtml('<div>hello!</div>');
  });
});
