const path = require('path')
const fs = require('fs')

// Add node fetch stuff (https://github.com/seanmonstar/reqwest/issues/910)
const entryFilePathNode = path.join(__dirname, '../node/identity_wasm.js')
const entryFileNode = fs.readFileSync(entryFilePathNode).toString()
let changedFileNode = entryFileNode.replace(
    "let imports = {};",
    "const fetch = require(\'node-fetch\')\r\nglobal.Headers = fetch.Headers\r\nglobal.Request = fetch.Request\r\nglobal.Response = fetch.Response\r\nglobal.fetch = fetch\r\n\r\nlet imports = {};"
)
fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
)

