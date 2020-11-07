import initWasm, * as wasm from './web/iota_identity_wasm'
let __initializedIotaWasm = false


export default function identity() {
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
    return identity().then(lib => new lib.Key())
}

/**
 * Generate a DID from a public key
 */
function newDID(publicKey) {
    return identity().then(lib => new lib.DID(publicKey))
}

export {
    newKey,
    newDID
}