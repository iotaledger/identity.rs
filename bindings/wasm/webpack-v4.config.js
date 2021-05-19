const path = require("path")
const CopyWebPlugin = require("copy-webpack-plugin")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")

const dist = path.resolve(__dirname, "dist")

module.exports = {
  mode: "production",
  entry: {
    index: "./examples/bundler-v4.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyWebPlugin([
      path.resolve(__dirname, "static"),
    ]),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: "pkg/bundler",
      outName: "identity_wasm",
    }),
  ],
  // Makes the output less verbose
  stats: 'minimal',
  // Removes the asset size warning
  performance: {
    hints: false,
  },
}
