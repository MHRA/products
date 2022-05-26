module.exports = {
  transform: {
    "^.+\\.jsx?$": `<rootDir>/config/jest-preprocess.js`,
  },
  testPathIgnorePatterns: [
    `node_modules`,
    `\\.cache`,
    `<rootDir>.*/public`,
    `<rootDir>/cypress`,
  ],
  transformIgnorePatterns: [`node_modules/(?!(gatsby|gatsby-script|uuid)/)`],
  moduleNameMapper: {
    ".+\\.(css|styl|less|sass|scss)$": `identity-obj-proxy`,
    ".+\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$": `<rootDir>/config/file-mock.js`,
  },
  testEnvironmentOptions: {
    url: `http://localhost`,
  },
  testEnvironment: "jsdom",
  globals: {
    __PATH_PREFIX__: ``,
  },
  setupFiles: [`<rootDir>/config/loadershim.js`],
}
