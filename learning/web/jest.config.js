module.exports = {
  transform: {
    "^.+\\.jsx?$": `<rootDir>/config/jest-preprocess.js`,
  },
  moduleNameMapper: {
    ".+\\.(css|styl|less|sass|scss)$": `identity-obj-proxy`,
    ".+\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga)$": `<rootDir>/config/file-mock.js`,
  },
  testPathIgnorePatterns: [
    `node_modules`,
    `\\.cache`,
    `<rootDir>.*/public`,
    `<rootDir>/cypress`,
  ],
  transformIgnorePatterns: [`node_modules/(?!(gatsby)/)`],
  globals: {
    __PATH_PREFIX__: ``,
  },
  roots: ["<rootDir>/src/", "<rootDir>/config/"],
  testURL: `http://localhost`,
  setupFiles: [`<rootDir>/config/loadershim.js`, "<rootDir>/config/enzyme.js"],
  snapshotSerializers: ["enzyme-to-json/serializer"],
}
