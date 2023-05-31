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

/** Runs all custom lints on the generated code. Exits the process immediately with code 1 if any fail. **/
function lintAll(content) {
    try {
        lintBigInt(content);
    } catch (err) {
        console.error("Custom lint failed!");
        console.error(err);
        process.exit(1);
    }
}

exports.lintAll = lintAll;
