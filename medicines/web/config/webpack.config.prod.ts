import webpack from 'webpack';
import config from './webpack.config.base';

const prodConfig: webpack.Configuration = {
  ...config,
  mode: 'production',
};

export default prodConfig;
