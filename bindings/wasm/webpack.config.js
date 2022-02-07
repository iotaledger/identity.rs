const path = require("path");
const CopyWebPlugin = require('copy-webpack-plugin');
const nodeExternals = require('webpack-node-externals');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./node/identity_wasm.js"
  },
  output: {
    path: dist,
    filename: 'index.mjs',
    module: true,
    chunkFormat: 'module',
    library: {
      type: 'module'
    },
    environment: {
      module: true
    }
  },
  target: 'node16',
  //externalsType: 'module',
  // resolve: {
  //   fallback: { "crypto": false }
  // },
  externalsPresets: { node: true },
  externals: {
    crypto: 'crypto',
    'node-fetch': 'node-fetch',
  },
  //externals: [nodeExternals({importType: "module"})], // in order to ignore all modules in node_modules folder
  plugins: [
    // new CopyWebPlugin({
    //   patterns: [
    //     {
    //       from: path.resolve(__dirname, "static")
    //     }
    //   ]
    // }),

  ],
  // // Makes the output less verbose
  // stats: 'minimal',
  // // Removes the asset size warning
  // performance: {
  //   hints: false,
  // },
  experiments: {
    asyncWebAssembly: true,
    outputModule: true,
  }
};
