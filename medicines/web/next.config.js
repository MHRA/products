require('dotenv').config();

module.exports = {
  env: {
    AZURE_SEARCH_API_VERSION: process.env.AZURE_SEARCH_API_VERSION,
    AZURE_SEARCH_INDEX: process.env.AZURE_SEARCH_INDEX,
    AZURE_SEARCH_KEY: process.env.AZURE_SEARCH_KEY,
    AZURE_SEARCH_SERVICE: process.env.AZURE_SEARCH_SERVICE,
    AZURE_SEARCH_WORD_FUZZINESS: process.env.AZURE_SEARCH_WORD_FUZZINESS,
    AZURE_SEARCH_SCORING_PROFILE: process.env.AZURE_SEARCH_SCORING_PROFILE,
    AZURE_SEARCH_EXACTNESS_BOOST: process.env.AZURE_SEARCH_EXACTNESS_BOOST,
    GA_TRACKING_ID:
      process.env.SOURCE_BRANCH === 'master'
        ? 'UA-6838115-11'
        : 'UA-6838115-11',
  },
  assetPrefix:
    process.env.SOURCE_BRANCH === 'master' ||
    process.env.SOURCE_BRANCH === undefined
      ? ''
      : `/${process.env.SOURCE_BRANCH}`,
};
