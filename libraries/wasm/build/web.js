const path = require('path')
const fs = require('fs')

const entryFilePath = path.join(__dirname, '../web/iota_identity_wasm.js')
const entryFile = fs.readFileSync(entryFilePath).toString()
// comment out this code so it works for Webpack
let changedFile = entryFile.replace(
  "input = import.meta.url.replace(",
  "// input = import.meta.url.replace("
)
  // Rename original init function, because we want to use the name for our own function
  .replace(
    "async function init(input) {",
    "async function initWasm(input) {"
  )
  .replace(
    "init.__wbindgen_wasm_module = module;",
    "initWasm.__wbindgen_wasm_module = module;"
  )
  // Create an init function which imports the wasm file
  .replace(
    "export default init;",
    "let __initializedIotaWasm = false\r\n\r\nexport function init() {\r\n    if (__initializedIotaWasm) {\r\n        return Promise.resolve(wasm)\r\n    }\r\n    return initWasm(\'iota_identity_wasm_bg.wasm\').then(() => {\r\n        __initializedIotaWasm = true\r\n        wasm.initialize()\r\n        return wasm\r\n    })\r\n}\r\n"
  )

fs.writeFileSync(
  entryFilePath,
  changedFile
)

