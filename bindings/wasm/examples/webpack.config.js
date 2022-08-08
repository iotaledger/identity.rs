const path = require('path');
const glob = require('glob');
const CopyWebPlugin = require('copy-webpack-plugin');
const serverConfig = {
    target: 'node',
    entry: './examples/src/node.js',
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'node.js',
    },
    externals: [
        function ({ context, request }, callback) {
            if (/^@iota\/identity-wasm$/.test(request)) {
                // Externalize to a commonjs module
                return callback(null, 'commonjs ' + path.resolve(__dirname, '../node'));
            }

            // Continue without externalizing the import
            callback();
        },
    ],
};

const serverTestConfig = {
    target: 'node',
    entry: glob.sync('./examples/src/tests/node/**.js').reduce(function(obj, el){
        obj[path.parse(el).name] = el;
        return obj
     },{}),
    devtool: "source-map",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'tests/node/[name].js'
    },
    externals: [
        function ({ context, request }, callback) {
            if (/^@iota\/identity-wasm$/.test(request)) {
                // Externalize to a commonjs module
                return callback(null, 'commonjs ' + path.resolve(__dirname, '../node'));
            }

            // Continue without externalizing the import
            callback();
        },
    ],
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
    },
    resolve: {
        alias: {
            '@iota/identity-wasm': path.resolve(__dirname, '../web'),
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
