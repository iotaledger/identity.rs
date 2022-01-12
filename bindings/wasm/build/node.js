const path = require('path')
const fs = require('fs')
const { lintBigInt } = require('./lints')

// Add node fetch stuff (https://github.com/seanmonstar/reqwest/issues/910)
const entryFilePathNode = path.join(__dirname, '../node/identity_wasm.js')
const entryFileNode = fs.readFileSync(entryFilePathNode).toString()

lintBigInt(entryFileNode);

let changedFileNode = entryFileNode.replace(
    "let imports = {};",
    `if (!global.is_fetch_polyfilled) {
        const fetch = require('node-fetch')
        global.Headers = fetch.Headers
        global.Request = fetch.Request
        global.Response = fetch.Response
        global.fetch = fetch
        global.is_fetch_polyfilled=true
    }
    let imports = {};`
)
fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
)

