const path = require('path');
const DtsBundleWebpack = require('dts-bundle-webpack');
const CircularDependencyPlugin = require('circular-dependency-plugin')
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: './wasm-node/index.js',
    target: 'node16',
    output: {
        path: path.resolve(__dirname),
        filename: 'index.unused.ts',
    },
    plugins: [
        new DtsBundleWebpack({
            name: 'identity-wasm',
            main: path.resolve(__dirname, "index.d.ts"),
            baseDir: path.resolve(__dirname),
            out: path.resolve(__dirname, '..', 'node', 'index.d.ts'),
            outputAsModuleFolder: true,
        }),
        new CircularDependencyPlugin(),
        new CopyPlugin({
            patterns: [
              { from: path.resolve(__dirname, '*[!webpack.config].js'), to: path.resolve(__dirname, '..', 'node', '[name].[ext]') },
              { from: path.resolve(__dirname, 'identity_wasm_bg.wasm' ), to: path.resolve(__dirname, '..', 'node') },
              { from: path.resolve(__dirname, 'package.json' ), to: path.resolve(__dirname, '..', 'node') },
            ],
          }),
    ],
};