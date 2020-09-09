import {
  buildFuzzyQuery,
  escapeSpecialWords,
  extractNormalizedProductLicenses,
} from './search-query-normalizer';

describe(buildFuzzyQuery, () => {
  beforeAll(() => {
    expect(process.env.AZURE_SEARCH_EXACTNESS_BOOST).toBe('4');
    expect(process.env.AZURE_SEARCH_WORD_FUZZINESS).toBe('1');
  });

  it('ignores special characters', () => {
    const fuzzyQuery = buildFuzzyQuery(
      'bacteriostatic solvent (0.3%w/v metacresol in wfi)',
    );
    expect(fuzzyQuery).toBe(
      '(bacteriostatic~1 || bacteriostatic^4) (solvent~1 || solvent^4) (0.3~1 || 0.3^4) (w~1 || w^4) (v~1 || v^4) (metacresol~1 || metacresol^4) (in~1 || in^4) (wfi~1 || wfi^4)',
    );
  });

  it('builds fuzzy words from product name', () => {
    const fuzzyQuery = buildFuzzyQuery('K/L POULTICE (KAOLIN POULTICE BP)');
    expect(fuzzyQuery).toBe(
      '(K~1 || K^4) (L~1 || L^4) (POULTICE~1 || POULTICE^4) (KAOLIN~1 || KAOLIN^4) (POULTICE~1 || POULTICE^4) (BP~1 || BP^4)',
    );
  });

  it('normalizes product licence', () => {
    const fuzzyQuery = buildFuzzyQuery('pl 30464/0140');
    expect(fuzzyQuery).toBe('(PL304640140~1 || PL304640140^4)');
  });

  it('extracts and normalizes product licence', () => {
    const fuzzyQuery = buildFuzzyQuery('amlodipine pl 30464/0140');
    expect(fuzzyQuery).toBe(
      '(amlodipine~1 || amlodipine^4) (PL304640140~1 || PL304640140^4)',
    );
  });
});

describe(extractNormalizedProductLicenses, () => {
  it.each`
    input                                               | expectedResult
    ${'pl 30464/0140'}                                  | ${'PL304640140'}
    ${'pl30464/0140'}                                   | ${'PL304640140'}
    ${'30464/0140'}                                     | ${'PL304640140'}
    ${'pl/30464/0140'}                                  | ${'PL304640140'}
    ${'pl-30464-0140'}                                  | ${'PL304640140'}
    ${'pl_30464_0140'}                                  | ${'PL304640140'}
    ${'hr 30464/0140'}                                  | ${'hr PL304640140'}
    ${'thr 30464/0140'}                                 | ${'thr PL304640140'}
    ${'pretext 30464/0140'}                             | ${'pretext PL304640140'}
    ${'pretext 30464-0140'}                             | ${'pretext PL304640140'}
    ${'pretext 30464_0140 posttext'}                    | ${'pretext posttext PL304640140'}
    ${'pretext 30464_0140 midtext 12345_1234 posttext'} | ${'pretext midtext posttext PL304640140 PL123451234'}
  `('converts $input to $expectedResult', ({ input, expectedResult }) => {
    const result = extractNormalizedProductLicenses(input);
    expect(result).toBe(expectedResult);
  });
});

describe(escapeSpecialWords, () => {
  it.each`
    input              | expectedResult
    ${'this AND that'} | ${'this \\AND that'}
    ${'this && that'}  | ${'this \\&& that'}
    ${'this OR that'}  | ${'this \\OR that'}
    ${'this || that'}  | ${'this \\|| that'}
    ${'this NOT that'} | ${'this \\NOT that'}
  `('converts $input to $expectedResult', ({ input, expectedResult }) => {
    const result = escapeSpecialWords(input);
    expect(result).toBe(expectedResult);
  });
});
