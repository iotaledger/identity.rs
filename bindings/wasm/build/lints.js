/** Aborts the build process if disallowed occurrences are found in identity_wasm.js **/
function lintBigInt(content) {
    if (content.includes("BigInt64Array") || content.includes("BigUint64Array")) {
        throw(
        "Build artifacts should not include BigInt64Array/BigUint64Array imports\n" +
        "to ensure React Native/WebKit compatibility.\n" +
        "Remove any u64 and i64 occurrence from the public Wasm interface.\n" +
        "See: https://github.com/iotaledger/identity.rs/issues/362\n"
        );
    }
}

/**
 * Rejects any `<obj>.ptr = 0;` occurrence, excluding `this.ptr = 0;` in `free()` implementations.
 *
 * Prevents generated code that nulls out Wasm pointers without de-registering the finalizer, since they cause
 * runtime errors during automatic garbage collection from WasmRefCell thinking the instance is still borrowed.
 *
 * Functions which take owned parameters cause this situation; the solution is to borrow and clone the parameter
 * instead.
 **/
function lintPtrNullWithoutFree(content) {
    // Find line numbers of offending code.
    const lines = content.split(/\r?\n/);
    const matches = lines.flatMap(function (line, number) {
        if (/(?<!this).ptr = 0;/.test(line)) {
            return [(number + 1) + " " + line.trim()];
        } else {
            return [];
        }
    });
    if (matches.length > 0) {
        throw(`ERROR: generated Javascript should not include 'obj.ptr = 0;'. 
When weak references are enabled with '--weak-refs', WasmRefCell in wasm-bindgen causes 
runtime errors from automatic garbage collection trying to free objects taken as owned parameters. 

Matches:
${matches}

SUGGESTION: change any exported functions which take an owned parameter (excluding flat enums) to use a borrow instead.
See: https://github.com/rustwasm/wasm-bindgen/pull/2677`);
    }
}

/** Runs all custom lints on the generated code. Exits the process immediately with code 1 if any fail. **/
function lintAll(content) {
    try {
        lintBigInt(content);
        lintPtrNullWithoutFree(content);
    } catch (err) {
        console.error("Custom lint failed!");
        console.error(err);
        process.exit(1);
    }
}

exports.lintAll = lintAll;
