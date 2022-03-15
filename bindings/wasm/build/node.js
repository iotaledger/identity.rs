const path = require('path')
const fs = require('fs')
const { lintAll } = require('./lints')

const entryFilePathNode = path.join(__dirname, '../node/identity_wasm.js');
const entryFileNode = fs.readFileSync(entryFilePathNode).toString();

lintAll(entryFileNode);

// Add node-fetch polyfill (https://github.com/seanmonstar/reqwest/issues/910).
let changedFileNode = entryFileNode.replace(
    "let imports = {};",
    `if (!globalThis.fetch) {
    const fetch = require('node-fetch')
    globalThis.Headers = fetch.Headers
    globalThis.Request = fetch.Request
    globalThis.Response = fetch.Response
    globalThis.fetch = fetch
}
let imports = {};`);

fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
);
