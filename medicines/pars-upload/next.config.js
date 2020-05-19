const withPlugins = require('next-compose-plugins');
const optimizedImages = require('next-optimized-images');

module.exports = withPlugins([
  [
    optimizedImages,
    {
      /* config for next-optimized-images */
    },
  ],

  // your other plugins here
]);

module.exports = {
  assetPrefix:
    process.env.NEXT_PUBLIC_IS_STATIC_BUILD === 'true' ? 'assets' : '',
};
