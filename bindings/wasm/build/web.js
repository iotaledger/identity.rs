const path = require('path');
const fs = require('fs');
const fse = require('fs-extra');
const { lintAll } = require('./lints');
const generatePackage = require('./utils/generatePackage');

const RELEASE_FOLDER = path.join(__dirname, '../web/');
const entryFilePath = path.join(RELEASE_FOLDER, 'identity_wasm.js');
const entryFile = fs.readFileSync(entryFilePath).toString();

lintAll(entryFile);

let changedFile = entryFile
    // Comment out generated code as a workaround for webpack (does not recognise import.meta).
    // Regex to avoid hard-coding 'identity_wasm_bg.wasm'.
    .replace(
        /input = new URL\((.*), import\.meta\.url\);/i,
        "// input = new URL($1, import.meta.url);"
    )
    // Rename original init function, because we want to use the name for our own function.
    .replace(
        "async function init(input) {",
        "async function initWasm(input) {"
    )
    .replace(
        "init.__wbindgen_wasm_module = module;",
        "initWasm.__wbindgen_wasm_module = module;"
    )
    // Create an init function which imports the wasm file.
    .replace(
        "export default init;",
        "let __initializedIotaWasm = false\r\n\r\nexport function init(path) {\r\n    if (__initializedIotaWasm) {\r\n        return Promise.resolve(wasm)\r\n    }\r\n    return initWasm(path || \'identity_wasm_bg.wasm\').then(() => {\r\n        __initializedIotaWasm = true\r\n        return wasm\r\n    })\r\n}\r\n"
    );

fs.writeFileSync(
    entryFilePath,
    changedFile
);

const entryFilePathTs = path.join(RELEASE_FOLDER, 'identity_wasm.d.ts');
const entryFileTs = fs.readFileSync(entryFilePathTs).toString();
// Replace the init function in the ts file.
let changedFileTs = entryFileTs.replace(
    "/**\n* If `module_or_path` is {RequestInfo} or {URL}, makes a request and\n* for everything else, calls `WebAssembly.instantiate` directly.\n*\n* @param {InitInput | Promise<InitInput>} module_or_path\n*\n* @returns {Promise<InitOutput>}\n*/\nexport default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;",
    "\/**\r\n* Loads the Wasm file so the lib can be used, relative path to Wasm file\r\n* @param {string | undefined} path\r\n*\/\r\nexport function init (path?: string): Promise<void>;"
);
fs.writeFileSync(
    entryFilePathTs,
    changedFileTs
);

// Copy TypeScript to a temporary directory (to avoid overwriting it with changes).
const tmpDir = path.join(__dirname, "..", "tmp");
fse.copySync(path.join(__dirname, '..', 'lib'), tmpDir, { 'overwrite': true });

// Replace `iota-client` import path in `stardust_identity_client.ts`.
// `@iota/client-wasm/node` -> `@iota/client-wasm/web`.
const clientFilePath = path.join(tmpDir, 'stardust_identity_client.ts');
const clientFileTs = fs.readFileSync(clientFilePath).toString()
    .replace(
        /from '(.*)\/node';/i,
        "from '$1/web';"
    );
fs.writeFileSync(
    clientFilePath,
    clientFileTs
);

// Generate `package.json`.
const newPackage = generatePackage({
    module: 'index.js',
    types: 'index.d.ts',
});
fs.writeFileSync(path.join(RELEASE_FOLDER, 'package.json'), JSON.stringify(newPackage, null, 2));
