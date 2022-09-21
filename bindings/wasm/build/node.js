const path = require("path");
const fs = require("fs");
const { lintAll } = require("./lints");
const generatePackage = require("./utils/generatePackage");

const RELEASE_FOLDER = path.join(__dirname, "../node/");
const entryFilePathNode = path.join(RELEASE_FOLDER, "identity_wasm.js");
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
let imports = {};`,
);

fs.writeFileSync(
    entryFilePathNode,
    changedFileNode,
);

// Generate `package.json`.
const newPackage = generatePackage({
    main: "index.js",
    types: "index.d.ts",
});
fs.writeFileSync(path.join(RELEASE_FOLDER, "package.json"), JSON.stringify(newPackage, null, 2));
