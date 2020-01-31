module.exports = {
  moduleFileExtensions: ['ts', 'tsx', 'js', 'md'],
  transform: {
    '^.+\\.tsx?$': 'ts-jest',
  },
  testMatch: ['**/*.(test|spec).(ts|tsx)'],
  globals: {
    'ts-jest': {
      babelConfig: '<rootDir>/.babelrc',
      tsConfig: '<rootDir>/jest.tsconfig.json',
    },
  },
  coveragePathIgnorePatterns: ['/node_modules/', 'enzyme.ts'],
  setupFilesAfterEnv: ['<rootDir>/config/enzyme.ts'],
  coverageReporters: ['json', 'lcov', 'text', 'text-summary'],
  moduleNameMapper: {
    '\\.(jpg|jpeg|png|gif|eot|otf|webp|svg|ttf|woff|woff2|mp4|webm|wav|mp3|m4a|aac|oga|md)$':
      '<rootDir>/config/assets-transformer.ts',
    '\\.(css|less|scss)$': '<rootDir>/config/assets-transformer.ts',
  },
};
