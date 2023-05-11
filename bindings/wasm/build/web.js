const path = require("path");
const fs = require("fs");
const fse = require("fs-extra");
const { lintAll } = require("./lints");
const generatePackage = require("./utils/generatePackage");

const RELEASE_FOLDER = path.join(__dirname, "../web/");
const entryFilePath = path.join(RELEASE_FOLDER, "identity_wasm.js");
const entryFile = fs.readFileSync(entryFilePath).toString();

lintAll(entryFile);

let changedFile = entryFile
    // Comment out generated code as a workaround for webpack (does not recognise import.meta).
    // Regex to avoid hard-coding 'identity_wasm_bg.wasm'.
    .replace(
        /input = new URL\((.*), import\.meta\.url\);/i,
        "// input = new URL($1, import.meta.url);",
    )
    // Create an init function which imports the wasm file.
    .concat(
        "let __initializedIotaWasm = false\r\n\r\nexport function init(path) {\r\n    if (__initializedIotaWasm) {\r\n        return Promise.resolve(wasm)\r\n    }\r\n    return __wbg_init(path || 'identity_wasm_bg.wasm').then(() => {\r\n        __initializedIotaWasm = true\r\n        return wasm\r\n    })\r\n}\r\n",
    );

fs.writeFileSync(
    entryFilePath,
    changedFile,
);

const entryFilePathTs = path.join(RELEASE_FOLDER, "identity_wasm.d.ts");
const entryFileTs = fs.readFileSync(entryFilePathTs).toString();

let changedFileTs = entryFileTs.concat(
    `
/**
* Loads the Wasm file so the lib can be used, relative path to Wasm file
*
* @param {string | undefined} path
*/
export function init (path?: string): Promise<void>;`,
);
fs.writeFileSync(
    entryFilePathTs,
    changedFileTs,
);
// Generate `package.json`.
const newPackage = generatePackage({
    module: "index.js",
    types: "index.d.ts",
});
fs.writeFileSync(path.join(RELEASE_FOLDER, "package.json"), JSON.stringify(newPackage, null, 2));
