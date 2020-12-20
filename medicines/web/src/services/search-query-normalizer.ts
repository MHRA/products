const searchExactnessBoost = process.env.AZURE_SEARCH_EXACTNESS_BOOST;
const searchWordFuzziness = process.env.AZURE_SEARCH_WORD_FUZZINESS;

export const productLicenseRegExp: RegExp = new RegExp(
  '(\\s)*(PL|PLGB|PLNI|THR|NR)(\\s+|/|_|-)*(\\d{5})(\\s+|/|_|-)*(\\d{4})',
  'ig',
);

const nonSearchableCharactersRegExp: RegExp = new RegExp(
  /(?:[,+\-!(){}\[\]^~*?:%\/]|\s+)/,
  'ig',
);

const specialWordsRegExp: RegExp = new RegExp(
  /(\|\||&&|\bAND\b|\bOR\b|\bNOT\b)/,
  'gi',
);

export const escapeSpecialWords = (word: string): string =>
  word.replace(specialWordsRegExp, `\\$1`);

const preferExactMatchButSupportFuzzyMatch = (word: string): string =>
  `(${word}~${searchWordFuzziness} || ${word}^${searchExactnessBoost})`;

export const normalizeProductLicenses = (q: string): string => {
  return q.replace(productLicenseRegExp, (match, p1, p2, p3, p4, p5, p6) => {
    return `${p1 ? p1 : ''}${p2.toUpperCase()}${p4}${p6}`;
  });
};

const splitByNonSearchableCharacters = (query: string) =>
  query.split(nonSearchableCharactersRegExp);

export const buildFuzzyQuery = (query: string): string => {
  return splitByNonSearchableCharacters(normalizeProductLicenses(query))
    .filter((x) => x.length > 0)
    .map((word) => escapeSpecialWords(word))
    .map((word) => preferExactMatchButSupportFuzzyMatch(word))
    .join(' ');
};
