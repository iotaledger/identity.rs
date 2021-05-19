// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const fs = require("fs")
const path = require("path")

const PKG_NODE = path.join(__dirname, "../pkg/node/identity_wasm.js")

const INPUT_STR = "let imports = {};"

const OUTPUT_STR = [
  "const fetch = require('node-fetch');",
  "global.Headers = fetch.Headers;",
  "global.Request = fetch.Request;",
  "global.Response = fetch.Response;",
  "global.fetch = fetch;",
  "",
  INPUT_STR,
].join("\r\n")

// Add `node-fetch` to resolve issues with the Rust `reqwest` crate.
//
// https://github.com/seanmonstar/reqwest/issues/910
function inject() {
  const input = fs.readFileSync(PKG_NODE).toString()
  const output = input.replace(INPUT_STR, OUTPUT_STR)

  fs.writeFileSync(PKG_NODE, output)
}

exports.inject = inject
