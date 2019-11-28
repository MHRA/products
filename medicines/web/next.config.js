const Dotenv = require('dotenv-webpack');

module.exports = {
  distDir: 'dist',
  poweredByHeader: false,
  webpack: config => {
    config.plugins.push(new Dotenv());
    return config;
  },
  // exportPathMap: function() {
  //   return {
  //     '/': { page: '/' },
  //   };
  // },
};
