const path = require('path');
const CopyWebPlugin = require('copy-webpack-plugin');
const serverConfig = {
    target: 'node16',
    entry: './examples/src/node.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'node.mjs',
        library: {
            type: 'module',
        },
    },
    experiments: {
        asyncWebAssembly: true,
        outputModule: true,
    },
    resolve: {
        alias: {
            '@iota/identity-wasm': path.resolve(__dirname, '../node/identity_wasm.js'),
        },
    },
};

const serverTestConfig = {
    target: 'node16',
    entry: './examples/src/test.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'test.mjs',
        library: {
            type: 'module',
        },
    },
    experiments: {
        asyncWebAssembly: true,
        outputModule: true,
    },
    resolve: {
        alias: {
            '@iota/identity-wasm': path.resolve(__dirname, '../node/identity_wasm.js'),
        },
    },
};

const clientConfig = {
    target: 'web',
    entry: './examples/src/web.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'web.js',
        library: {
            type: 'module',
        },
    },
    experiments: {
        topLevelAwait: true,
        outputModule: true,
        asyncWebAssembly: true,
    },
    resolve: {
        alias: {
            '@iota/identity-wasm': path.resolve(__dirname, '../web/identity_wasm.js'),
        },
    },
    plugins: [
        new CopyWebPlugin({
            patterns: [
                {
                    from: path.resolve(__dirname, "./src/index.html"),
                    to: path.resolve(__dirname, "dist"),
                },

                {
                    from: path.resolve(__dirname, "../web/identity_wasm_bg.wasm"),
                    to: path.resolve(__dirname, "dist"),
                }
            ]

        }),
    ],
};

module.exports = [serverConfig, serverTestConfig, clientConfig];
