import initWasm, * as wasm from './web/iota_identity_wasm'
let __initializedIotaWasm = false


export default function identity() {
    if (__initializedIotaWasm) {
        return Promise.resolve(wasm)
    }
    return initWasm('iota_identity_wasm_bg.wasm').then(() => {
        __initializedIotaWasm = true
        wasm.initialize()
        return wasm
    })
}
