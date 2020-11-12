const fs = require('fs')
const path = require('path')
const jsdoc2md = require('jsdoc-to-markdown')

const importNode = path.join(__dirname, '../node/iota_identity_wasm.js')
const importWeb = path.join(__dirname, '../web/iota_identity_wasm.js')

const exportNode = path.join(__dirname, '../docs/api-reference-node.md')
const exportWeb = path.join(__dirname, '../docs/api-reference-web.md')

const docsRoot = path.join(__dirname, '../docs')

const docsNode = jsdoc2md.renderSync({ files: importNode })
const docsWeb = jsdoc2md.renderSync({ files: importWeb })

if (!fs.existsSync(docsRoot)) {
  fs.mkdirSync(docsRoot)
}

fs.writeFileSync(exportNode, docsNode)
fs.writeFileSync(exportWeb, docsWeb)
