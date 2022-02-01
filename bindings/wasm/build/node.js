const path = require('path')
const fs = require('fs')
const { lintBigInt } = require('./lints')

// Add node fetch stuff (https://github.com/seanmonstar/reqwest/issues/910)
const entryFilePathNode = path.join(__dirname, '../node/identity_wasm.js')
const entryFileNode = fs.readFileSync(entryFilePathNode).toString()

lintBigInt(entryFileNode);

let changedFileNode = entryFileNode.replace(
    "wasm.__wbindgen_start();",
    `import crypto from 'crypto'
    if (!globalThis.crypto) {
        globalThis.crypto = crypto
    }
    import fetch, {Headers, Request, Response} from 'node-fetch'
    globalThis.crypto = crypto
    if (!globalThis.fetch) {
        globalThis.Headers = Headers
        globalThis.Request = Request
        globalThis.Response = Response
        globalThis.fetch = fetch
    }
    wasm.__wbindgen_start();`
)
fs.writeFileSync(
    entryFilePathNode,
    changedFileNode
)

const bgFilePathNode = path.join(__dirname, '../node/identity_wasm_bg.js')
const bgFileNode = fs.readFileSync(bgFilePathNode).toString()

lintBigInt(bgFileNode);

let changedBgFileNode = bgFileNode.replace(
    "var ret = module.require(getStringFromWasm0(arg0, arg1));",
`
console.log(getStringFromWasm0(arg0, arg1));
console.log(globalThis);
if (getStringFromWasm0(arg0, arg1) === "crypto") { 
  return addHeapObject(globalThis.crypto)
} 
var ret = module.require(getStringFromWasm0(arg0, arg1));`
)
fs.writeFileSync(
    bgFilePathNode,
    changedBgFileNode
)
