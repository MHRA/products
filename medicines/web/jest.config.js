module.exports = {
  coveragePathIgnorePatterns: ['/node_modules/', 'enzyme.ts'],
  coverageReporters: ['json', 'lcov', 'text', 'text-summary'],
  moduleFileExtensions: ['ts', 'tsx', 'js', 'md'],
  globals: {
    'ts-jest': {
      babelConfig: '<rootDir>/.babelrc',
      tsConfig: '<rootDir>/jest.tsconfig.json',
    },
  },
  moduleNameMapper: {
    '\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga|md)$':
      '<rootDir>/config/assets-transformer.ts',
    '\\.(css|less|scss)$': '<rootDir>/config/assets-transformer.ts',
  },
  setupFilesAfterEnv: ['<rootDir>/config/enzyme.ts'],
  snapshotSerializers: ['enzyme-to-json/serializer'],
  testMatch: ['**/*.(test|spec).(ts|tsx)'],
  transform: {
    '^.+\\.tsx?$': 'ts-jest',
  },
};
