const path = require('path');

const DtsBundleWebpack = require('dts-bundle-webpack');
const CircularDependencyPlugin = require('circular-dependency-plugin');
const CopyPlugin = require("copy-webpack-plugin");

const srcDir = path.resolve(__dirname);
const distDir = path.resolve(__dirname, '..', 'web');

module.exports = {
    entry: path.resolve(srcDir, "index.js"),
    target: 'web',
    output: {
        path: srcDir,
        filename: 'index.unused.ts',
    },
    plugins: [
        new DtsBundleWebpack({
            name: 'identity-wasm',
            main: path.resolve(srcDir, "index.d.ts"),
            baseDir: srcDir,
            out: path.resolve(distDir, 'index.d.ts'),
            outputAsModuleFolder: true,
        }),
        new CircularDependencyPlugin(),
        new CopyPlugin({
            patterns: [
              { from: path.resolve(srcDir, '*[!webpack.config].js'), to: path.resolve(distDir, '[name].[ext]') },
              { from: path.resolve(srcDir, 'identity_wasm_bg.wasm' ), to: distDir },
              { from: path.resolve(srcDir, 'package.json' ), to: distDir },
            ],
          }),
    ],
};