const path = require('path')
const fs = require('fs')

const entryFilePath = path.join(__dirname, '../web/iota_identity_wasm.js')
const entryFile = fs.readFileSync(entryFilePath).toString()

// comment out this code so it works for Webpack
fs.writeFileSync(
  entryFilePath,
  entryFile.replace(
    "input = import.meta.url.replace(",
    "// input = import.meta.url.replace("
  )
)

// Create an init function which imports the wasm file
fs.writeFileSync(
  entryFilePath,
  entryFile.replace(
    "export default init;",
    "let __initializedIotaWasm = false\r\n\r\n\r\nfunction initLib() {\r\n    if (__initializedIotaWasm) {\r\n        return Promise.resolve(wasm)\r\n    }\r\n    return init(\'iota_identity_wasm_bg.wasm\').then(() => {\r\n        __initializedIotaWasm = true\r\n        wasm.initialize()\r\n        return wasm\r\n    })\r\n}\r\n\r\nexport default initLib;"
  )
)