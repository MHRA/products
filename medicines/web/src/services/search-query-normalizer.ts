const searchExactnessBoost = process.env.AZURE_SEARCH_EXACTNESS_BOOST;
const searchWordFuzziness = process.env.AZURE_SEARCH_WORD_FUZZINESS;

const extractProductLicenseRegExp: RegExp = new RegExp(
  '(\\b|PL)(\\s+|/|_|-)*(\\d{5})(\\s+|/|_|-)*(\\d{4})',
  'ig',
);

const escapeSpecialWords = (word: string): string =>
  word.replace(/(\|\||&&|\bAND\b|\bOR\b|\bNOT\b)/gi, `\\$1`);

const preferExactMatchButSupportFuzzyMatch = (word: string): string =>
  `(${word}~${searchWordFuzziness}||${word}^${searchExactnessBoost})`;

const extractNormalizedProductLicenses = (q: string): string => {
  const normalizedProductLicences = q
    .match(extractProductLicenseRegExp)
    ?.map(match => match.replace(extractProductLicenseRegExp, 'PL$3$5'));

  if (normalizedProductLicences && normalizedProductLicences.length) {
    const normalizedProductLicencesString: string = normalizedProductLicences.join(
      ' ',
    );
    const qWithoutProductLicences = q.replace(extractProductLicenseRegExp, '');
    return `${qWithoutProductLicences} ${normalizedProductLicencesString}`;
  }

  return `${q}`;
};

const splitByNonSearchableCharacters = (query: string) =>
  query.split(/(?:[,+\-!(){}\[\]^~*?:%\/]|\s+)/gi);

export const buildFuzzyQuery = (query: string): string => {
  return splitByNonSearchableCharacters(extractNormalizedProductLicenses(query))
    .filter(x => x.length > 0)
    .map(word => escapeSpecialWords(word))
    .map(word => preferExactMatchButSupportFuzzyMatch(word))
    .join(' ');
};
