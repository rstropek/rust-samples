const HTMLWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.ts",
  experiments: {
    syncWebAssembly: true
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  mode: "development",
  plugins: [
    new HTMLWebpackPlugin({
      template: path.resolve(__dirname, 'src/index.html'),
    }),
  ],
};
