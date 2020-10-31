import initWasm, * as wasm from './web/iota_identity_wasm'
let __initializedIotaWasm = false


function __getLib() {
    if (__initializedIotaWasm) {
        return Promise.resolve(wasm)
    }
    return initWasm('iota_identity_wasm_bg.wasm').then(() => {
        __initializedIotaWasm = true
        return wasm
    })
}

// Todo add all functions

/**
 * Get a Ed25519 keypair
 */
function newKey() {
    return __getLib().then(lib => new lib.Key())
}

/**
 * Generate a DID from a public key
 */
function newDID(publicKey) {
    return __getLib().then(lib => new lib.DID(publicKey))
}

export {
    newKey,
    newDID
}