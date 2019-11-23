import Dotenv from 'dotenv-webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import path from 'path';
import webpack from 'webpack';

const sourceIndex = path.resolve(__dirname, '../src/index.tsx');
export const distPath = path.resolve(__dirname, '../dist');

const config: webpack.Configuration = {
  entry: ['whatwg-fetch', sourceIndex],
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  output: {
    filename: 'bundle.js',
    path: distPath,
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'MHRA Medicines Information Portal',
      filename: 'index.html',
      template: 'src/index.html',
    }),
    new Dotenv(),
  ],
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
};

export default config;
