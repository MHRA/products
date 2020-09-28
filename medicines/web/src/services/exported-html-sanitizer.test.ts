import { getCleanedHtml, getHtmlBody } from './exported-html-sanitizer';

describe(getCleanedHtml, () => {
  it('extracts body', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE><html><head></head><body><div>Hi there!</div></body></html>',
      '',
    );
    expect(output).toBe('<div>Hi there!</div>');
  });
  it('replaces asset image tags', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><img src="/img1.jpg" /></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<img src="https://report-assets/img1.jpg">');
  });
  it('removes unwanted tags', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><div><h1>title</h1><o:p></o:p><script>some dangerous javascript</script></div></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<div></div>');
  });
  it('removes unwanted table attributes', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><table id="test" style="color: black" v:shapes="" cellspacing="2" cellpadding="10" border="1" width="300" valign="top" class="test"></table></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<table id="test"></table>');
  });
  it('removes unwanted attributes', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><div id="test" style="color: black" align="left" class="test"></div></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<div id="test"></div>');
  });
  it('replaces anchor name with id', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><a name="link" href="/link">Link text</a></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<a id="link" href="/link">Link text</a>');
  });
  it('replaces w:sdt tags with divs', () => {
    const output = getCleanedHtml(
      '<!DOCTYPE html><html><head></head><body><w:sdt><div><w:sdtpr></w:sdtpr>Content</div></w:sdt></body></html>',
      'https://report-assets/',
    );
    expect(output).toBe('<div><div><div></div>Content</div></div>');
  });
});
