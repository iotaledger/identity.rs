/** Rejects any `imports['env']` occurrences, which cause failures at runtime.
 *
 *  This is typically due to Wasm compatibility features not being enabled on crate dependencies. **/
function lintImportEnv(content) {
    if (content.includes("imports['env']") || content.includes("require('env')") || content.includes("from 'env'")) {
        throw (`ERROR: generated Javascript should not include any imports for 'env', e.g.:

- imports['env'] = require('env'); 
- imports['env'] = __wbg_star0;
- import * as __wbg_star0 from 'env';

It causes runtime errors such as "Module not found: Error: Can't resolve 'env'".
This usually indicates a dependency trying to use or import non-Wasm-compatible code or libraries.

Common problematic crates and the feature flag required to be compatible:
- parking_lot <= 0.11.2, "wasm-bindgen" (0.12.0 deprecated the "wasm-bindgen" feature).
- instant 0.1, "wasm-bindgen".
- getrandom 0.2, "js".

SUGGESTION: Identify the problematic crate by comparing recent changes to Cargo.toml in this project and any
dependencies. Then, enable the relevant "js" or "wasm-bindgen" feature flag in Cargo.toml for that specific
dependency and version.

E.g. (only add this to Cargo.toml if they appear in Cargo.lock, and the version must match Cargo.lock).
getrandom = { version = "0.2", default-features = false, features = ["js"] }
instant = { version = "0.1", default-features = false, features = ["wasm-bindgen"] }
 
See: 
- https://github.com/rustwasm/wasm-bindgen/issues/2160
- https://github.com/rustwasm/wasm-pack/issues/743`);
    }
}

/** Runs all custom lints on the generated code. Exits the process immediately with code 1 if any fail. **/
function lintAll(content) {
    try {
        lintImportEnv(content);
    } catch (err) {
        console.error("Custom lint failed!");
        console.error(err);
        process.exit(1);
    }
}

exports.lintAll = lintAll;
