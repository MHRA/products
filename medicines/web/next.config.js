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
    GOOGLE_GTM_CONTAINER_ID: process.env.GOOGLE_GTM_CONTAINER_ID,
    GOOGLE_TRACKING_ID: process.env.GOOGLE_TRACKING_ID,
    GOOGLE_USE_DEBUG: process.env.GOOGLE_USE_DEBUG,
    ROOT_URL_DOMAIN: process.env.ROOT_URL_DOMAIN,
    GRAPHQL_URL: process.env.GRAPHQL_URL,
    USE_GRAPHQL: process.env.USE_GRAPHQL,
    BMGF_AZURE_SEARCH_INDEX: process.env.BMGF_AZURE_SEARCH_INDEX,
    SHOW_BMGF: process.env.SHOW_BMGF,
    ENV: process.env.ENV,
  },
  webpack: (config) => {
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
  assetPrefix:
    process.env.ASSET_PREFIX === 'master' ||
    process.env.ASSET_PREFIX === undefined
      ? ''
      : `/${process.env.ASSET_PREFIX}`,
  trailingSlash: true,
};
