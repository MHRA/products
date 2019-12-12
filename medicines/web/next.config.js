require('dotenv').config();

const baseUrl =
  process.env.ASSET_PREFIX === 'master' ||
  process.env.ASSET_PREFIX === undefined
    ? ''
    : `/${process.env.ASSET_PREFIX}`;

module.exports = {
  env: {
    AZURE_SEARCH_API_VERSION: process.env.AZURE_SEARCH_API_VERSION,
    AZURE_SEARCH_INDEX: process.env.AZURE_SEARCH_INDEX,
    AZURE_SEARCH_KEY: process.env.AZURE_SEARCH_KEY,
    AZURE_SEARCH_SERVICE: process.env.AZURE_SEARCH_SERVICE,
    AZURE_SEARCH_WORD_FUZZINESS: process.env.AZURE_SEARCH_WORD_FUZZINESS,
    AZURE_SEARCH_SCORING_PROFILE: process.env.AZURE_SEARCH_SCORING_PROFILE,
    AZURE_SEARCH_EXACTNESS_BOOST: process.env.AZURE_SEARCH_EXACTNESS_BOOST,
    GOOGLE_GTM_CONTAINER_ID: process.env.GOOGLE_GTM_CONTAINER_ID,
    GOOGLE_TRACKING_ID: process.env.GOOGLE_TRACKING_ID,
  },
  webpack: config => {
    config.module.rules.push({
      test: /\.(md)$/,
      use: [
        { loader: 'html-loader' },
        {
          loader: 'markdown-loader',
        },
      ],
    });
    return config;
  },
  exportTrailingSlash: true,
  assetPrefix: baseUrl,
  publicRuntimeConfig: {
    baseUrl,
  },
};
