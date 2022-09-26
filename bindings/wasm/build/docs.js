const fs = require("fs");
const path = require("path");
const jsdoc2md = require("jsdoc-to-markdown");

const importFile = path.join(__dirname, "../node/identity_wasm.js");
const exportFile = path.join(__dirname, "../docs/api-reference.md");

const docsRoot = path.join(__dirname, "../docs");
const docsData = jsdoc2md.renderSync({ files: importFile });

if (!fs.existsSync(docsRoot)) {
    fs.mkdirSync(docsRoot);
}

fs.writeFileSync(exportFile, docsData);
