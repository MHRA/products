// This webpack configuration gets around problem of importing woff and woff2 files for the govuk-frontend package
// As suggested: https://dev.to/harveyjones282/the-simplest-way-to-configure-next-js-with-sass-3en
module.exports = {
  webpack(config) {
    config.module.rules.push({
      test: /\.(png|jpg|gif|svg|eot|ttf|woff|woff2)$/,
      use: {
        loader: 'url-loader',
        options: {
          limit: 100000,
        },
      },
    })

    return config
  },
}
