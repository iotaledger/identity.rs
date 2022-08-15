const path = require('path');

const DtsBundleWebpack = require('dts-bundle-webpack');
const CircularDependencyPlugin = require('circular-dependency-plugin');
const CopyPlugin = require("copy-webpack-plugin");

const srcDir = path.resolve(__dirname).replace(/\\/g, "/");
const distDir = path.resolve(__dirname, '..', 'web').replace(/\\/g, "/");

module.exports = {
    entry: `${srcDir}/index.js`,
    target: 'web',
    output: {
        path: srcDir,
        filename: 'index.unused.ts',
    },
    plugins: [
        new DtsBundleWebpack({
            name: 'identity-wasm',
            main: `${srcDir}/index.d.ts`,
            baseDir: srcDir,
            out: `${distDir}/index.d.ts`,
            outputAsModuleFolder: true,
        }),
        new CircularDependencyPlugin(),
        new CopyPlugin({
            patterns: [
                { from: `${srcDir}/*[!webpack.config].js`, to: `${distDir}/[name].[ext]` },
                { from: `${srcDir}/identity_wasm_bg.wasm`, to: distDir },
                { from: `${srcDir}/package.json`, to: distDir },
            ],
        }),
    ],
};