import webpack from 'webpack';
import config, { distPath } from './webpack.config.base';

const devConfig: webpack.Configuration = {
  ...config,
  devtool: 'inline-source-map',
  mode: 'development',
  devServer: {
    contentBase: distPath,
    port: 3000,
  },
};

export default devConfig;
