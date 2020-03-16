import { buildFuzzyQuery } from './search-query-normalizer';

describe(buildFuzzyQuery, () => {
  beforeAll(() => {
    expect(process.env.AZURE_SEARCH_EXACTNESS_BOOST).toBe('4');
    expect(process.env.AZURE_SEARCH_WORD_FUZZINESS).toBe('1');
  });

  it('ignores special characters', () => {
    const fuzzyQuery = buildFuzzyQuery('hello*');
    expect(fuzzyQuery).toBe('hello~1 hello^4');
  });

  it('builds fuzzy words from product name', () => {
    const fuzzyQuery = buildFuzzyQuery('K/L POULTICE (KAOLIN POULTICE BP)');
    expect(fuzzyQuery).toBe(
      'K~1 K^4 L~1 L^4 POULTICE~1 POULTICE^4 KAOLIN~1 KAOLIN^4 POULTICE~1 POULTICE^4 BP~1 BP^4',
    );
  });

  it('normalizes product licence', () => {
    const fuzzyQuery = buildFuzzyQuery('pl 30464/0140');
    expect(fuzzyQuery).toBe('PL304640140~1 PL304640140^4');
  });

  it('extracts and normalizes product licence', () => {
    const fuzzyQuery = buildFuzzyQuery('amlodipine pl 30464/0140');
    expect(fuzzyQuery).toBe(
      'amlodipine~1 amlodipine^4 PL304640140~1 PL304640140^4',
    );
  });
});
