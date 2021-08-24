
// Aborts the build process if disallowed occurences are found in identity_wasm.js
function lintBigInt(content) {
    if (content.includes("BigInt64Array") || content.includes("BigUint64Array")) {
        console.error("Build artifacts should not include BigInt64Array/BigUint64Array imports")
        console.error("to ensure React Native/WebKit compatibility.")
        console.error("Remove any u64 and i64 occurrence from the public Wasm interface.")
        console.error("See: https://github.com/iotaledger/identity.rs/issues/362")
        process.exit(1)
    }
}

exports.lintBigInt = lintBigInt;
