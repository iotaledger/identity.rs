
if (!globalThis.fetch) {
    const fetch = require('node-fetch')
    globalThis.Headers = fetch.Headers
    globalThis.Request = fetch.Request
    globalThis.Response = fetch.Response
    globalThis.fetch = fetch
}
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder, TextEncoder, inspect } = require(`util`);

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
    : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for (let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(state => {
        wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
    });

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hecbb567bf7d4b77d(arg0, arg1, addHeapObject(arg2));
}

/**
* Initializes the console error panic hook for better error messages
*/
module.exports.start = function () {
    wasm.start();
};

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* Verify a JWS signature secured with the `EdDSA` algorithm and curve `Ed25519`.
*
* This function is useful when one is composing a `IJwsVerifier` that delegates
* `EdDSA` verification with curve `Ed25519` to this function.
*
* # Warning
*
* This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
* prior to calling the function.
* @param {JwsAlgorithm} alg
* @param {Uint8Array} signingInput
* @param {Uint8Array} decodedSignature
* @param {Jwk} publicKey
*/
module.exports.verifyEd25519 = function (alg, signingInput, decodedSignature, publicKey) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(signingInput, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(decodedSignature, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        _assertClass(publicKey, Jwk);
        wasm.verifyEd25519(retptr, addHeapObject(alg), ptr0, len0, ptr1, len1, publicKey.__wbg_ptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
};

/**
* Encode the given bytes in url-safe base64.
* @param {Uint8Array} data
* @returns {string}
*/
module.exports.encodeB64 = function (data) {
    let deferred2_0;
    let deferred2_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.encodeB64(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred2_0 = r0;
        deferred2_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
};

/**
* Decode the given url-safe base64-encoded slice into its raw bytes.
* @param {Uint8Array} data
* @returns {Uint8Array}
*/
module.exports.decodeB64 = function (data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.decodeB64(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
};

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_813(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h58660668f6114e02(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
*/
module.exports.SerializationType = Object.freeze({ COMPACT: 0, "0": "COMPACT", JSON: 1, "1": "JSON", });
/**
* Purpose of a {@link StatusList2021}.
*/
module.exports.StatusPurpose = Object.freeze({ Revocation: 0, "0": "Revocation", Suspension: 1, "1": "Suspension", });
/**
* Declares how credential subjects must relate to the presentation holder.
*
* See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.
*/
module.exports.SubjectHolderRelationship = Object.freeze({
    /**
    * The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
    * This variant is the default.
    */
    AlwaysSubject: 0, "0": "AlwaysSubject",
    /**
    * The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.
    */
    SubjectOnNonTransferable: 1, "1": "SubjectOnNonTransferable",
    /**
    * The holder is not required to have any kind of relationship to any credential subject.
    */
    Any: 2, "2": "Any",
});
/**
* Controls validation behaviour when checking whether or not a credential has been revoked by its
* [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
*/
module.exports.StatusCheck = Object.freeze({
    /**
    * Validate the status if supported, reject any unsupported
    * [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
    *
    * Only `RevocationBitmap2022` is currently supported.
    *
    * This is the default.
    */
    Strict: 0, "0": "Strict",
    /**
    * Validate the status if supported, skip any unsupported
    * [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
    */
    SkipUnsupported: 1, "1": "SkipUnsupported",
    /**
    * Skip all status checks.
    */
    SkipAll: 2, "2": "SkipAll",
});
/**
*/
module.exports.ProofAlgorithm = Object.freeze({ BLS12381_SHA256: 0, "0": "BLS12381_SHA256", BLS12381_SHAKE256: 1, "1": "BLS12381_SHAKE256", SU_ES256: 2, "2": "SU_ES256", MAC_H256: 3, "3": "MAC_H256", MAC_H384: 4, "4": "MAC_H384", MAC_H512: 5, "5": "MAC_H512", MAC_K25519: 6, "6": "MAC_K25519", MAC_K448: 7, "7": "MAC_K448", MAC_H256K: 8, "8": "MAC_H256K", });
/**
* Declares when validation should return if an error occurs.
*/
module.exports.FailFast = Object.freeze({
    /**
    * Return all errors that occur during validation.
    */
    AllErrors: 0, "0": "AllErrors",
    /**
    * Return after the first error occurs.
    */
    FirstError: 1, "1": "FirstError",
});
/**
*/
module.exports.StateMetadataEncoding = Object.freeze({ Json: 0, "0": "Json", });
/**
*/
module.exports.MethodRelationship = Object.freeze({ Authentication: 0, "0": "Authentication", AssertionMethod: 1, "1": "AssertionMethod", KeyAgreement: 2, "2": "KeyAgreement", CapabilityDelegation: 3, "3": "CapabilityDelegation", CapabilityInvocation: 4, "4": "CapabilityInvocation", });
/**
*/
module.exports.CredentialStatus = Object.freeze({ Revoked: 0, "0": "Revoked", Suspended: 1, "1": "Suspended", Valid: 2, "2": "Valid", });
/**
*/
module.exports.PayloadType = Object.freeze({ Disclosed: 0, "0": "Disclosed", Undisclosed: 1, "1": "Undisclosed", ProofMethods: 2, "2": "ProofMethods", });
/**
*/
module.exports.PresentationProofAlgorithm = Object.freeze({ BLS12381_SHA256_PROOF: 0, "0": "BLS12381_SHA256_PROOF", BLS12381_SHAKE256_PROOF: 1, "1": "BLS12381_SHAKE256_PROOF", SU_ES256: 2, "2": "SU_ES256", MAC_H256: 3, "3": "MAC_H256", MAC_H384: 4, "4": "MAC_H384", MAC_H512: 5, "5": "MAC_H512", MAC_K25519: 6, "6": "MAC_K25519", MAC_K448: 7, "7": "MAC_K448", MAC_H256K: 8, "8": "MAC_H256K", });

const CoreDIDFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_coredid_free(ptr >>> 0, 1));
/**
* A method-agnostic Decentralized Identifier (DID).
*/
class CoreDID {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CoreDID.prototype);
        obj.__wbg_ptr = ptr;
        CoreDIDFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CoreDIDFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_coredid_free(ptr, 0);
    }
    /**
    * Parses a {@link CoreDID} from the given `input`.
    *
    * ### Errors
    *
    * Throws an error if the input is not a valid {@link CoreDID}.
    * @param {string} input
    * @returns {CoreDID}
    */
    static parse(input) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.coredid_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the method name of the {@link CoreDID}.
    * @param {string} value
    */
    setMethodName(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.coredid_setMethodName(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Validates whether a string is a valid DID method name.
    * @param {string} value
    * @returns {boolean}
    */
    static validMethodName(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.coredid_validMethodName(ptr0, len0);
        return ret !== 0;
    }
    /**
    * Set the method-specific-id of the `DID`.
    * @param {string} value
    */
    setMethodId(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.coredid_setMethodId(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Validates whether a string is a valid `DID` method-id.
    * @param {string} value
    * @returns {boolean}
    */
    static validMethodId(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.coredid_validMethodId(ptr0, len0);
        return ret !== 0;
    }
    /**
    * Returns the {@link CoreDID} scheme.
    *
    * E.g.
    * - `"did:example:12345678" -> "did"`
    * - `"did:iota:smr:12345678" -> "did"`
    * @returns {string}
    */
    scheme() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_scheme(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the {@link CoreDID} authority: the method name and method-id.
    *
    * E.g.
    * - `"did:example:12345678" -> "example:12345678"`
    * - `"did:iota:smr:12345678" -> "iota:smr:12345678"`
    * @returns {string}
    */
    authority() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_authority(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the {@link CoreDID} method name.
    *
    * E.g.
    * - `"did:example:12345678" -> "example"`
    * - `"did:iota:smr:12345678" -> "iota"`
    * @returns {string}
    */
    method() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_method(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the {@link CoreDID} method-specific ID.
    *
    * E.g.
    * - `"did:example:12345678" -> "12345678"`
    * - `"did:iota:smr:12345678" -> "smr:12345678"`
    * @returns {string}
    */
    methodId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_methodId(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Construct a new {@link DIDUrl} by joining with a relative DID Url string.
    * @param {string} segment
    * @returns {DIDUrl}
    */
    join(segment) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(segment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.coredid_join(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DIDUrl.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Clones the {@link CoreDID} into a {@link DIDUrl}.
    * @returns {DIDUrl}
    */
    toUrl() {
        const ret = wasm.coredid_toUrl(this.__wbg_ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Converts the {@link CoreDID} into a {@link DIDUrl}, consuming it.
    * @returns {DIDUrl}
    */
    intoUrl() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.coredid_intoUrl(ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Returns the {@link CoreDID} as a string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {CoreDID}
    */
    toCoreDid() {
        const ret = wasm.coredid_clone(this.__wbg_ptr);
        return CoreDID.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {CoreDID}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredid_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {CoreDID}
    */
    clone() {
        const ret = wasm.coredid_clone(this.__wbg_ptr);
        return CoreDID.__wrap(ret);
    }
}
module.exports.CoreDID = CoreDID;

const CoreDocumentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_coredocument_free(ptr >>> 0, 1));
/**
* A method-agnostic DID Document.
*
* Note: All methods that involve reading from this class may potentially raise an error
* if the object is being concurrently modified.
*/
class CoreDocument {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CoreDocument.prototype);
        obj.__wbg_ptr = ptr;
        CoreDocumentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CoreDocumentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_coredocument_free(ptr, 0);
    }
    /**
    * Creates a new {@link CoreDocument} with the given properties.
    * @param {ICoreDocument} values
    */
    constructor(values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_new(retptr, addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            CoreDocumentFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the DID Document `id`.
    * @returns {CoreDID}
    */
    id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the DID of the document.
    *
    * ### Warning
    *
    * Changing the identifier can drastically alter the results of
    * `resolve_method`, `resolve_service` and the related
    * [DID URL dereferencing](https://w3c-ccg.github.io/did-resolution/#dereferencing) algorithm.
    * @param {CoreDID} id
    */
    setId(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(id, CoreDID);
            wasm.coredocument_setId(retptr, this.__wbg_ptr, id.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document controllers.
    * @returns {Array<CoreDID>}
    */
    controller() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_controller(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the controllers of the DID Document.
    *
    * Note: Duplicates will be ignored.
    * Use `null` to remove all controllers.
    * @param {CoreDID | CoreDID[] | null} controllers
    */
    setController(controllers) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_setController(retptr, this.__wbg_ptr, addBorrowedObject(controllers));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a copy of the document's `alsoKnownAs` set.
    * @returns {Array<string>}
    */
    alsoKnownAs() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_alsoKnownAs(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `alsoKnownAs` property in the DID document.
    * @param {string | string[] | null} urls
    */
    setAlsoKnownAs(urls) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_setAlsoKnownAs(retptr, this.__wbg_ptr, addBorrowedObject(urls));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a copy of the document's `verificationMethod` set.
    * @returns {VerificationMethod[]}
    */
    verificationMethod() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_verificationMethod(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document's `authentication` set.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    authentication() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_authentication(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document's `assertionMethod` set.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    assertionMethod() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_assertionMethod(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document's `keyAgreement` set.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    keyAgreement() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_keyAgreement(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document's `capabilityDelegation` set.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    capabilityDelegation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_capabilityDelegation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the document's `capabilityInvocation` set.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    capabilityInvocation() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_capabilityInvocation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the custom DID Document properties.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a custom property in the DID Document.
    * If the value is set to `null`, the custom property will be removed.
    *
    * ### WARNING
    *
    * This method can overwrite existing properties like `id` and result in an invalid document.
    * @param {string} key
    * @param {any} value
    */
    setPropertyUnchecked(key, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.coredocument_setPropertyUnchecked(retptr, this.__wbg_ptr, ptr0, len0, addBorrowedObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a set of all {@link Service} in the document.
    * @returns {Service[]}
    */
    service() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_service(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Add a new {@link Service} to the document.
    *
    * Errors if there already exists a service or verification method with the same id.
    * @param {Service} service
    */
    insertService(service) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(service, Service);
            wasm.coredocument_insertService(retptr, this.__wbg_ptr, service.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Remove a {@link Service} identified by the given {@link DIDUrl} from the document.
    *
    * Returns `true` if the service was removed.
    * @param {DIDUrl} didUrl
    * @returns {Service | undefined}
    */
    removeService(didUrl) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(didUrl, DIDUrl);
            wasm.coredocument_removeService(retptr, this.__wbg_ptr, didUrl.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the first {@link Service} with an `id` property matching the provided `query`,
    * if present.
    * @param {DIDUrl | string} query
    * @returns {Service | undefined}
    */
    resolveService(query) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_resolveService(retptr, this.__wbg_ptr, addBorrowedObject(query));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a list of all {@link VerificationMethod} in the DID Document,
    * whose verification relationship matches `scope`.
    *
    * If `scope` is not set, a list over the **embedded** methods is returned.
    * @param {MethodScope | undefined} [scope]
    * @returns {VerificationMethod[]}
    */
    methods(scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_methods(retptr, this.__wbg_ptr, isLikeNone(scope) ? 0 : addHeapObject(scope));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns an array of all verification relationships.
    * @returns {Array<DIDUrl | VerificationMethod>}
    */
    verificationRelationships() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_verificationRelationships(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds a new `method` to the document in the given `scope`.
    * @param {VerificationMethod} method
    * @param {MethodScope} scope
    */
    insertMethod(method, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(method, VerificationMethod);
            _assertClass(scope, MethodScope);
            wasm.coredocument_insertMethod(retptr, this.__wbg_ptr, method.__wbg_ptr, scope.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Removes all references to the specified Verification Method.
    * @param {DIDUrl} did
    * @returns {VerificationMethod | undefined}
    */
    removeMethod(did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, DIDUrl);
            wasm.coredocument_removeMethod(retptr, this.__wbg_ptr, did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the first verification method with an `id` property
    * matching the provided `query` and the verification relationship
    * specified by `scope`, if present.
    * @param {DIDUrl | string} query
    * @param {MethodScope | undefined} [scope]
    * @returns {VerificationMethod | undefined}
    */
    resolveMethod(query, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_resolveMethod(retptr, this.__wbg_ptr, addBorrowedObject(query), isLikeNone(scope) ? 0 : addHeapObject(scope));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Attaches the relationship to the given method, if the method exists.
    *
    * Note: The method needs to be in the set of verification methods,
    * so it cannot be an embedded one.
    * @param {DIDUrl} didUrl
    * @param {MethodRelationship} relationship
    * @returns {boolean}
    */
    attachMethodRelationship(didUrl, relationship) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(didUrl, DIDUrl);
            wasm.coredocument_attachMethodRelationship(retptr, this.__wbg_ptr, didUrl.__wbg_ptr, relationship);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Detaches the given relationship from the given method, if the method exists.
    * @param {DIDUrl} didUrl
    * @param {MethodRelationship} relationship
    * @returns {boolean}
    */
    detachMethodRelationship(didUrl, relationship) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(didUrl, DIDUrl);
            wasm.coredocument_detachMethodRelationship(retptr, this.__wbg_ptr, didUrl.__wbg_ptr, relationship);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Decodes and verifies the provided JWS according to the passed `options` and `signatureVerifier`.
    *  If no `signatureVerifier` argument is provided a default verifier will be used that is (only) capable of
    * verifying EdDSA signatures.
    *
    * Regardless of which options are passed the following conditions must be met in order for a verification attempt to
    * take place.
    * - The JWS must be encoded according to the JWS compact serialization.
    * - The `kid` value in the protected header must be an identifier of a verification method in this DID document,
    * or set explicitly in the `options`.
    * @param {Jws} jws
    * @param {JwsVerificationOptions} options
    * @param {IJwsVerifier} signatureVerifier
    * @param {string | undefined} [detachedPayload]
    * @returns {DecodedJws}
    */
    verifyJws(jws, options, signatureVerifier, detachedPayload) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(jws, Jws);
            _assertClass(options, JwsVerificationOptions);
            var ptr0 = isLikeNone(detachedPayload) ? 0 : passStringToWasm0(detachedPayload, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.coredocument_verifyJws(retptr, this.__wbg_ptr, jws.__wbg_ptr, options.__wbg_ptr, addHeapObject(signatureVerifier), ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJws.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * If the document has a {@link RevocationBitmap} service identified by `serviceQuery`,
    * revoke all specified `indices`.
    * @param {DIDUrl | string} serviceQuery
    * @param {number | number[]} indices
    */
    revokeCredentials(serviceQuery, indices) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_revokeCredentials(retptr, this.__wbg_ptr, addBorrowedObject(serviceQuery), addHeapObject(indices));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * If the document has a {@link RevocationBitmap} service identified by `serviceQuery`,
    * unrevoke all specified `indices`.
    * @param {DIDUrl | string} serviceQuery
    * @param {number | number[]} indices
    */
    unrevokeCredentials(serviceQuery, indices) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_unrevokeCredentials(retptr, this.__wbg_ptr, addBorrowedObject(serviceQuery), addHeapObject(indices));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the {@link CoreDocument}.
    * @returns {CoreDocument}
    */
    clone() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_clone(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * ### Warning
    * This is for internal use only. Do not rely on or call this method.
    * @returns {CoreDocument}
    */
    _shallowCloneInternal() {
        const ret = wasm.coredocument__shallowCloneInternal(this.__wbg_ptr);
        return CoreDocument.__wrap(ret);
    }
    /**
    * ### Warning
    * This is for internal use only. Do not rely on or call this method.
    * @returns {number}
    */
    _strongCountInternal() {
        const ret = wasm.coredocument__strongCountInternal(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Serializes to a plain JS representation.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a plain JS representation.
    * @param {any} json
    * @returns {CoreDocument}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.coredocument_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Generate new key material in the given `storage` and insert a new verification method with the corresponding
    * public key material into the DID document.
    *
    * - If no fragment is given the `kid` of the generated JWK is used, if it is set, otherwise an error is returned.
    * - The `keyType` must be compatible with the given `storage`. `Storage`s are expected to export key type constants
    * for that use case.
    *
    * The fragment of the generated method is returned.
    * @param {Storage} storage
    * @param {string} keyType
    * @param {JwsAlgorithm} alg
    * @param {string | undefined} fragment
    * @param {MethodScope} scope
    * @returns {Promise<string>}
    */
    generateMethod(storage, keyType, alg, fragment, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(keyType, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(fragment) ? 0 : passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            _assertClass(scope, MethodScope);
            var ptr2 = scope.__destroy_into_raw();
            wasm.coredocument_generateMethod(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, addHeapObject(alg), ptr1, len1, ptr2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Remove the method identified by the `fragment` from the document and delete the corresponding key material in
    * the `storage`.
    * @param {Storage} storage
    * @param {DIDUrl} id
    * @returns {Promise<void>}
    */
    purgeMethod(storage, id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            _assertClass(id, DIDUrl);
            wasm.coredocument_purgeMethod(retptr, this.__wbg_ptr, storage.__wbg_ptr, id.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
    * material in the verification method identified by the given `fragment.
    *
    * Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
    * See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).
    * @param {Storage} storage
    * @param {string} fragment
    * @param {string} payload
    * @param {JwsSignatureOptions} options
    * @returns {Promise<Jws>}
    */
    createJws(storage, fragment, payload, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(payload, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwsSignatureOptions);
            wasm.coredocument_createJws(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, ptr1, len1, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Produces a JWT where the payload is produced from the given `credential`
    * in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
    *
    * Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
    * of the method identified by `fragment` and the JWS signature will be produced by the corresponding
    * private key backed by the `storage` in accordance with the passed `options`.
    *
    * The `custom_claims` can be used to set additional claims on the resulting JWT.
    * @param {Storage} storage
    * @param {string} fragment
    * @param {Credential} credential
    * @param {JwsSignatureOptions} options
    * @param {Record<string, any> | undefined} [custom_claims]
    * @returns {Promise<Jwt>}
    */
    createCredentialJwt(storage, fragment, credential, options, custom_claims) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(credential, Credential);
            _assertClass(options, JwsSignatureOptions);
            wasm.coredocument_createCredentialJwt(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, credential.__wbg_ptr, options.__wbg_ptr, isLikeNone(custom_claims) ? 0 : addHeapObject(custom_claims));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Produces a JWT where the payload is produced from the given presentation.
    * in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
    *
    * Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
    * of the method identified by `fragment` and the JWS signature will be produced by the corresponding
    * private key backed by the `storage` in accordance with the passed `options`.
    * @param {Storage} storage
    * @param {string} fragment
    * @param {Presentation} presentation
    * @param {JwsSignatureOptions} signature_options
    * @param {JwtPresentationOptions} presentation_options
    * @returns {Promise<Jwt>}
    */
    createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(presentation, Presentation);
            _assertClass(signature_options, JwsSignatureOptions);
            _assertClass(presentation_options, JwtPresentationOptions);
            wasm.coredocument_createPresentationJwt(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, presentation.__wbg_ptr, signature_options.__wbg_ptr, presentation_options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.CoreDocument = CoreDocument;

const CredentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_credential_free(ptr >>> 0, 1));
/**
*/
class Credential {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Credential.prototype);
        obj.__wbg_ptr = ptr;
        CredentialFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CredentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_credential_free(ptr, 0);
    }
    /**
    * Returns the base JSON-LD context.
    * @returns {string}
    */
    static BaseContext() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_BaseContext(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Returns the base type.
    * @returns {string}
    */
    static BaseType() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_BaseType(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Constructs a new {@link Credential}.
    * @param {ICredential} values
    */
    constructor(values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_new(retptr, addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            CredentialFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {IDomainLinkageCredential} values
    * @returns {Credential}
    */
    static createDomainLinkageCredential(values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_createDomainLinkageCredential(retptr, addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Credential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the JSON-LD context(s) applicable to the {@link Credential}.
    * @returns {Array<string | Record<string, any>>}
    */
    context() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_context(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the unique `URI` identifying the {@link Credential} .
    * @returns {string | undefined}
    */
    id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the URIs defining the type of the {@link Credential}.
    * @returns {Array<string>}
    */
    type() {
        const ret = wasm.credential_type(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns a copy of the {@link Credential} subject(s).
    * @returns {Array<Subject>}
    */
    credentialSubject() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_credentialSubject(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the issuer of the {@link Credential}.
    * @returns {string | Issuer}
    */
    issuer() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_issuer(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the timestamp of when the {@link Credential} becomes valid.
    * @returns {Timestamp}
    */
    issuanceDate() {
        const ret = wasm.credential_issuanceDate(this.__wbg_ptr);
        return Timestamp.__wrap(ret);
    }
    /**
    * Returns a copy of the timestamp of when the {@link Credential} should no longer be considered valid.
    * @returns {Timestamp | undefined}
    */
    expirationDate() {
        const ret = wasm.credential_expirationDate(this.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * Returns a copy of the information used to determine the current status of the {@link Credential}.
    * @returns {Array<Status>}
    */
    credentialStatus() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_credentialStatus(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the information used to assist in the enforcement of a specific {@link Credential} structure.
    * @returns {Array<Schema>}
    */
    credentialSchema() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_credentialSchema(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the service(s) used to refresh an expired {@link Credential}.
    * @returns {Array<RefreshService>}
    */
    refreshService() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_refreshService(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the terms-of-use specified by the {@link Credential} issuer.
    * @returns {Array<Policy>}
    */
    termsOfUse() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_termsOfUse(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the human-readable evidence used to support the claims within the {@link Credential}.
    * @returns {Array<Evidence>}
    */
    evidence() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_evidence(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns whether or not the {@link Credential} must only be contained within a  {@link Presentation}
    * with a proof issued from the {@link Credential} subject.
    * @returns {boolean | undefined}
    */
    nonTransferable() {
        const ret = wasm.credential_nonTransferable(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * Optional cryptographic proof, unrelated to JWT.
    * @returns {Proof | undefined}
    */
    proof() {
        const ret = wasm.credential_proof(this.__wbg_ptr);
        return ret === 0 ? undefined : Proof.__wrap(ret);
    }
    /**
    * Returns a copy of the miscellaneous properties on the {@link Credential}.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `proof` property of the {@link Credential}.
    *
    * Note that this proof is not related to JWT.
    * @param {Proof | undefined} [proof]
    */
    setProof(proof) {
        let ptr0 = 0;
        if (!isLikeNone(proof)) {
            _assertClass(proof, Proof);
            ptr0 = proof.__destroy_into_raw();
        }
        wasm.credential_setProof(this.__wbg_ptr, ptr0);
    }
    /**
    * Serializes the `Credential` as a JWT claims set
    * in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
    *
    * The resulting object can be used as the payload of a JWS when issuing the credential.
    * @param {Record<string, any> | undefined} [custom_claims]
    * @returns {Record<string, any>}
    */
    toJwtClaims(custom_claims) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_toJwtClaims(retptr, this.__wbg_ptr, isLikeNone(custom_claims) ? 0 : addHeapObject(custom_claims));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Credential}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.credential_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Credential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Credential}
    */
    clone() {
        const ret = wasm.credential_clone(this.__wbg_ptr);
        return Credential.__wrap(ret);
    }
}
module.exports.Credential = Credential;

const CustomMethodDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_custommethoddata_free(ptr >>> 0, 1));
/**
* A custom verification method data format.
*/
class CustomMethodData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(CustomMethodData.prototype);
        obj.__wbg_ptr = ptr;
        CustomMethodDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CustomMethodDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_custommethoddata_free(ptr, 0);
    }
    /**
    * @param {string} name
    * @param {any} data
    */
    constructor(name, data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.custommethoddata_new(retptr, ptr0, len0, addHeapObject(data));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            CustomMethodDataFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deep clones the object.
    * @returns {CustomMethodData}
    */
    clone() {
        const ret = wasm.custommethoddata_clone(this.__wbg_ptr);
        return CustomMethodData.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.custommethoddata_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {CustomMethodData}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.custommethoddata_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CustomMethodData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.CustomMethodData = CustomMethodData;

const DIDUrlFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_didurl_free(ptr >>> 0, 1));
/**
* A method agnostic DID Url.
*/
class DIDUrl {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DIDUrl.prototype);
        obj.__wbg_ptr = ptr;
        DIDUrlFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DIDUrlFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_didurl_free(ptr, 0);
    }
    /**
    * Parses a {@link DIDUrl} from the input string.
    * @param {string} input
    * @returns {DIDUrl}
    */
    static parse(input) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.didurl_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DIDUrl.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Return a copy of the {@link CoreDID} section of the {@link DIDUrl}.
    * @returns {CoreDID}
    */
    did() {
        const ret = wasm.didurl_did(this.__wbg_ptr);
        return CoreDID.__wrap(ret);
    }
    /**
    * Return a copy of the relative DID Url as a string, including only the path, query, and fragment.
    * @returns {string}
    */
    urlStr() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_urlStr(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the {@link DIDUrl} method fragment, if any. Excludes the leading '#'.
    * @returns {string | undefined}
    */
    fragment() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_fragment(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `fragment` component of the {@link DIDUrl}.
    * @param {string | undefined} [value]
    */
    setFragment(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = isLikeNone(value) ? 0 : passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.didurl_setFragment(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the {@link DIDUrl} path.
    * @returns {string | undefined}
    */
    path() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_path(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `path` component of the {@link DIDUrl}.
    * @param {string | undefined} [value]
    */
    setPath(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = isLikeNone(value) ? 0 : passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.didurl_setPath(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the {@link DIDUrl} method query, if any. Excludes the leading '?'.
    * @returns {string | undefined}
    */
    query() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_query(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `query` component of the {@link DIDUrl}.
    * @param {string | undefined} [value]
    */
    setQuery(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = isLikeNone(value) ? 0 : passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.didurl_setQuery(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Append a string representing a path, query, and/or fragment, returning a new {@link DIDUrl}.
    *
    * Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
    * segment and any following segments in order of path, query, then fragment.
    *
    * I.e.
    * - joining a path will clear the query and fragment.
    * - joining a query will clear the fragment.
    * - joining a fragment will only overwrite the fragment.
    * @param {string} segment
    * @returns {DIDUrl}
    */
    join(segment) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(segment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.didurl_join(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DIDUrl.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the {@link DIDUrl} as a string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {DIDUrl}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.didurl_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DIDUrl.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {DIDUrl}
    */
    clone() {
        const ret = wasm.didurl_clone(this.__wbg_ptr);
        return DIDUrl.__wrap(ret);
    }
}
module.exports.DIDUrl = DIDUrl;

const DecodedJptCredentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_decodedjptcredential_free(ptr >>> 0, 1));
/**
*/
class DecodedJptCredential {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DecodedJptCredential.prototype);
        obj.__wbg_ptr = ptr;
        DecodedJptCredentialFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecodedJptCredentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decodedjptcredential_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {DecodedJptCredential}
    */
    clone() {
        const ret = wasm.decodedjptcredential_clone(this.__wbg_ptr);
        return DecodedJptCredential.__wrap(ret);
    }
    /**
    * Returns the {@link Credential} embedded into this JPT.
    * @returns {Credential}
    */
    credential() {
        const ret = wasm.decodedjptcredential_credential(this.__wbg_ptr);
        return Credential.__wrap(ret);
    }
    /**
    * Returns the custom claims parsed from the JPT.
    * @returns {Map<string, any>}
    */
    customClaims() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjptcredential_customClaims(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {JwpIssued}
    */
    decodedJwp() {
        const ret = wasm.decodedjptcredential_decodedJwp(this.__wbg_ptr);
        return JwpIssued.__wrap(ret);
    }
}
module.exports.DecodedJptCredential = DecodedJptCredential;

const DecodedJptPresentationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_decodedjptpresentation_free(ptr >>> 0, 1));
/**
*/
class DecodedJptPresentation {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DecodedJptPresentation.prototype);
        obj.__wbg_ptr = ptr;
        DecodedJptPresentationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecodedJptPresentationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decodedjptpresentation_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {DecodedJptPresentation}
    */
    clone() {
        const ret = wasm.decodedjptpresentation_clone(this.__wbg_ptr);
        return DecodedJptPresentation.__wrap(ret);
    }
    /**
    * Returns the {@link Credential} embedded into this JPT.
    * @returns {Credential}
    */
    credential() {
        const ret = wasm.decodedjptpresentation_credential(this.__wbg_ptr);
        return Credential.__wrap(ret);
    }
    /**
    * Returns the custom claims parsed from the JPT.
    * @returns {Map<string, any>}
    */
    customClaims() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjptpresentation_customClaims(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the `aud` property parsed from the JWT claims.
    * @returns {string | undefined}
    */
    aud() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjptpresentation_aud(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.DecodedJptPresentation = DecodedJptPresentation;

const DecodedJwsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_decodedjws_free(ptr >>> 0, 1));
/**
* A cryptographically verified decoded token from a JWS.
*
* Contains the decoded headers and the raw claims.
*/
class DecodedJws {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DecodedJws.prototype);
        obj.__wbg_ptr = ptr;
        DecodedJwsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecodedJwsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decodedjws_free(ptr, 0);
    }
    /**
    * Returns a copy of the parsed claims represented as a string.
    *
    * # Errors
    * An error is thrown if the claims cannot be represented as a string.
    *
    * This error can only occur if the Token was decoded from a detached payload.
    * @returns {string}
    */
    claims() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjws_claims(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Return a copy of the parsed claims represented as an array of bytes.
    * @returns {Uint8Array}
    */
    claimsBytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjws_claimsBytes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the protected header.
    * @returns {JwsHeader}
    */
    protectedHeader() {
        const ret = wasm.decodedjws_protectedHeader(this.__wbg_ptr);
        return JwsHeader.__wrap(ret);
    }
    /**
    * Deep clones the object.
    * @returns {DecodedJws}
    */
    clone() {
        const ret = wasm.decodedjws_clone(this.__wbg_ptr);
        return DecodedJws.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjws_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.DecodedJws = DecodedJws;

const DecodedJwtCredentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_decodedjwtcredential_free(ptr >>> 0, 1));
/**
* A cryptographically verified and decoded Credential.
*
* Note that having an instance of this type only means the JWS it was constructed from was verified.
* It does not imply anything about a potentially present proof property on the credential itself.
*/
class DecodedJwtCredential {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DecodedJwtCredential.prototype);
        obj.__wbg_ptr = ptr;
        DecodedJwtCredentialFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecodedJwtCredentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decodedjwtcredential_free(ptr, 0);
    }
    /**
    * Returns a copy of the credential parsed to the [Verifiable Credentials Data model](https://www.w3.org/TR/vc-data-model/).
    * @returns {Credential}
    */
    credential() {
        const ret = wasm.decodedjwtcredential_credential(this.__wbg_ptr);
        return Credential.__wrap(ret);
    }
    /**
    * Returns a copy of the protected header parsed from the decoded JWS.
    * @returns {JwsHeader}
    */
    protectedHeader() {
        const ret = wasm.decodedjwtcredential_protectedHeader(this.__wbg_ptr);
        return JwsHeader.__wrap(ret);
    }
    /**
    * The custom claims parsed from the JWT.
    * @returns {Record<string, any> | undefined}
    */
    customClaims() {
        const ret = wasm.decodedjwtcredential_customClaims(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Consumes the object and returns the decoded credential.
    *
    * ### Warning
    *
    * This destroys the {@link DecodedJwtCredential} object.
    * @returns {Credential}
    */
    intoCredential() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.decodedjwtcredential_intoCredential(ptr);
        return Credential.__wrap(ret);
    }
}
module.exports.DecodedJwtCredential = DecodedJwtCredential;

const DecodedJwtPresentationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_decodedjwtpresentation_free(ptr >>> 0, 1));
/**
* A cryptographically verified and decoded presentation.
*
* Note that having an instance of this type only means the JWS it was constructed from was verified.
* It does not imply anything about a potentially present proof property on the presentation itself.
*/
class DecodedJwtPresentation {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DecodedJwtPresentation.prototype);
        obj.__wbg_ptr = ptr;
        DecodedJwtPresentationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DecodedJwtPresentationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_decodedjwtpresentation_free(ptr, 0);
    }
    /**
    * @returns {Presentation}
    */
    presentation() {
        const ret = wasm.decodedjwtpresentation_presentation(this.__wbg_ptr);
        return Presentation.__wrap(ret);
    }
    /**
    * Returns a copy of the protected header parsed from the decoded JWS.
    * @returns {JwsHeader}
    */
    protectedHeader() {
        const ret = wasm.decodedjwtpresentation_protectedHeader(this.__wbg_ptr);
        return JwsHeader.__wrap(ret);
    }
    /**
    * Consumes the object and returns the decoded presentation.
    *
    * ### Warning
    * This destroys the {@link DecodedJwtPresentation} object.
    * @returns {Presentation}
    */
    intoPresentation() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.decodedjwtpresentation_intoPresentation(ptr);
        return Presentation.__wrap(ret);
    }
    /**
    * The expiration date parsed from the JWT claims.
    * @returns {Timestamp | undefined}
    */
    expirationDate() {
        const ret = wasm.decodedjwtpresentation_expirationDate(this.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * The issuance date parsed from the JWT claims.
    * @returns {Timestamp | undefined}
    */
    issuanceDate() {
        const ret = wasm.decodedjwtpresentation_issuanceDate(this.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * The `aud` property parsed from JWT claims.
    * @returns {string | undefined}
    */
    audience() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.decodedjwtpresentation_audience(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * The custom claims parsed from the JWT.
    * @returns {Record<string, any> | undefined}
    */
    customClaims() {
        const ret = wasm.decodedjwtpresentation_customClaims(this.__wbg_ptr);
        return takeObject(ret);
    }
}
module.exports.DecodedJwtPresentation = DecodedJwtPresentation;

const DisclosureFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_disclosure_free(ptr >>> 0, 1));
/**
* Represents an elements constructing a disclosure.
* Object properties and array elements disclosures are supported.
*
* See: https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-disclosures
*/
class Disclosure {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Disclosure.prototype);
        obj.__wbg_ptr = ptr;
        DisclosureFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DisclosureFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_disclosure_free(ptr, 0);
    }
    /**
    * @param {string} salt
    * @param {string | undefined} claim_name
    * @param {any} claim_value
    */
    constructor(salt, claim_name, claim_value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(salt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(claim_name) ? 0 : passStringToWasm0(claim_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.disclosure_new(retptr, ptr0, len0, ptr1, len1, addHeapObject(claim_value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            DisclosureFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Parses a Base64 encoded disclosure into a `Disclosure`.
    *
    * ## Error
    *
    * Returns an `InvalidDisclosure` if input is not a valid disclosure.
    * @param {string} disclosure
    * @returns {Disclosure}
    */
    static parse(disclosure) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(disclosure, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.disclosure_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Disclosure.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the base64url-encoded string.
    * @returns {string}
    */
    disclosure() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_disclosure(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the base64url-encoded string.
    * @returns {string}
    */
    toEncodedString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_disclosure(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the base64url-encoded string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_disclosure(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the salt value.
    * @returns {string}
    */
    salt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_salt(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the claim name, optional for array elements.
    * @returns {string | undefined}
    */
    claimName() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_claimName(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the claim Value which can be of any type.
    * @returns {any}
    */
    claimValue() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_claimValue(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Disclosure}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.disclosure_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Disclosure.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.Disclosure = Disclosure;

const DomainLinkageConfigurationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_domainlinkageconfiguration_free(ptr >>> 0, 1));
/**
* DID Configuration Resource which contains Domain Linkage Credentials.
* It can be placed in an origin's `.well-known` directory to prove linkage between the origin and a DID.
* See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>
*
* Note:
* - Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)
*/
class DomainLinkageConfiguration {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DomainLinkageConfiguration.prototype);
        obj.__wbg_ptr = ptr;
        DomainLinkageConfigurationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DomainLinkageConfigurationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_domainlinkageconfiguration_free(ptr, 0);
    }
    /**
    * Constructs a new {@link DomainLinkageConfiguration}.
    * @param {Array<Jwt>} linkedDids
    */
    constructor(linkedDids) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.domainlinkageconfiguration_new(retptr, addBorrowedObject(linkedDids));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            DomainLinkageConfigurationFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * List of the Domain Linkage Credentials.
    * @returns {Array<Jwt>}
    */
    linkedDids() {
        const ret = wasm.domainlinkageconfiguration_linkedDids(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * List of the issuers of the Domain Linkage Credentials.
    * @returns {Array<CoreDID>}
    */
    issuers() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.domainlinkageconfiguration_issuers(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.domainlinkageconfiguration_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {DomainLinkageConfiguration}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.domainlinkageconfiguration_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DomainLinkageConfiguration.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {DomainLinkageConfiguration}
    */
    clone() {
        const ret = wasm.domainlinkageconfiguration_clone(this.__wbg_ptr);
        return DomainLinkageConfiguration.__wrap(ret);
    }
}
module.exports.DomainLinkageConfiguration = DomainLinkageConfiguration;

const DurationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_duration_free(ptr >>> 0, 1));
/**
* A span of time.
*/
class Duration {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Duration.prototype);
        obj.__wbg_ptr = ptr;
        DurationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DurationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_duration_free(ptr, 0);
    }
    /**
    * Create a new {@link Duration} with the given number of seconds.
    * @param {number} seconds
    * @returns {Duration}
    */
    static seconds(seconds) {
        const ret = wasm.duration_seconds(seconds);
        return Duration.__wrap(ret);
    }
    /**
    * Create a new {@link Duration} with the given number of minutes.
    * @param {number} minutes
    * @returns {Duration}
    */
    static minutes(minutes) {
        const ret = wasm.duration_minutes(minutes);
        return Duration.__wrap(ret);
    }
    /**
    * Create a new {@link Duration} with the given number of hours.
    * @param {number} hours
    * @returns {Duration}
    */
    static hours(hours) {
        const ret = wasm.duration_hours(hours);
        return Duration.__wrap(ret);
    }
    /**
    * Create a new {@link Duration} with the given number of days.
    * @param {number} days
    * @returns {Duration}
    */
    static days(days) {
        const ret = wasm.duration_days(days);
        return Duration.__wrap(ret);
    }
    /**
    * Create a new {@link Duration} with the given number of weeks.
    * @param {number} weeks
    * @returns {Duration}
    */
    static weeks(weeks) {
        const ret = wasm.duration_weeks(weeks);
        return Duration.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.duration_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Duration}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.duration_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Duration.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.Duration = Duration;

const EdDSAJwsVerifierFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_eddsajwsverifier_free(ptr >>> 0, 1));
/**
* An implementor of `IJwsVerifier` that can handle the
* `EdDSA` algorithm.
*/
class EdDSAJwsVerifier {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EdDSAJwsVerifierFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eddsajwsverifier_free(ptr, 0);
    }
    /**
    * Constructs an EdDSAJwsVerifier.
    */
    constructor() {
        const ret = wasm.eddsajwsverifier_new();
        this.__wbg_ptr = ret >>> 0;
        EdDSAJwsVerifierFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Verify a JWS signature secured with the `EdDSA` algorithm.
    * Only the `Ed25519` curve is supported for now.
    *
    * This function is useful when one is building an `IJwsVerifier` that extends the default provided by
    * the IOTA Identity Framework.
    *
    * # Warning
    *
    * This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
    * prior to calling the function.
    * @param {JwsAlgorithm} alg
    * @param {Uint8Array} signingInput
    * @param {Uint8Array} decodedSignature
    * @param {Jwk} publicKey
    */
    verify(alg, signingInput, decodedSignature, publicKey) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(signingInput, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArray8ToWasm0(decodedSignature, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(publicKey, Jwk);
            wasm.eddsajwsverifier_verify(retptr, this.__wbg_ptr, addHeapObject(alg), ptr0, len0, ptr1, len1, publicKey.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.EdDSAJwsVerifier = EdDSAJwsVerifier;

const IotaDIDFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_iotadid_free(ptr >>> 0, 1));
/**
* A DID conforming to the IOTA DID method specification.
*
* @typicalname did
*/
class IotaDID {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IotaDID.prototype);
        obj.__wbg_ptr = ptr;
        IotaDIDFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
            METHOD: this.METHOD,
            DEFAULT_NETWORK: this.DEFAULT_NETWORK,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IotaDIDFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_iotadid_free(ptr, 0);
    }
    /**
    * The IOTA DID method name (`"iota"`).
    * @returns {string}
    */
    static get METHOD() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_static_default_network(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * The default Tangle network (`"iota"`).
    * @returns {string}
    */
    static get DEFAULT_NETWORK() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_static_default_network(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Constructs a new {@link IotaDID} from a byte representation of the tag and the given
    * network name.
    *
    * See also {@link IotaDID.placeholder}.
    * @param {Uint8Array} bytes
    * @param {string} network
    */
    constructor(bytes, network) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(network, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.iotadid_new(retptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            IotaDIDFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Constructs a new {@link IotaDID} from a hex representation of an Alias Id and the given
    * network name.
    * @param {string} aliasId
    * @param {string} network
    * @returns {IotaDID}
    */
    static fromAliasId(aliasId, network) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(aliasId, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(network, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.iotadid_fromAliasId(retptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Creates a new placeholder {@link IotaDID} with the given network name.
    *
    * E.g. `did:iota:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.
    * @param {string} network
    * @returns {IotaDID}
    */
    static placeholder(network) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(network, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadid_placeholder(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Parses a {@link IotaDID} from the input string.
    * @param {string} input
    * @returns {IotaDID}
    */
    static parse(input) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadid_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the Tangle network name of the {@link IotaDID}.
    * @returns {string}
    */
    network() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_network(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the unique tag of the {@link IotaDID}.
    * @returns {string}
    */
    tag() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_tag(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the DID represented as a {@link CoreDID}.
    * @returns {CoreDID}
    */
    toCoreDid() {
        const ret = wasm.iotadid_toCoreDid(this.__wbg_ptr);
        return CoreDID.__wrap(ret);
    }
    /**
    * Returns the `DID` scheme.
    *
    * E.g.
    * - `"did:example:12345678" -> "did"`
    * - `"did:iota:main:12345678" -> "did"`
    * @returns {string}
    */
    scheme() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_scheme(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the `DID` authority: the method name and method-id.
    *
    * E.g.
    * - `"did:example:12345678" -> "example:12345678"`
    * - `"did:iota:main:12345678" -> "iota:main:12345678"`
    * @returns {string}
    */
    authority() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_authority(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the `DID` method name.
    *
    * E.g.
    * - `"did:example:12345678" -> "example"`
    * - `"did:iota:main:12345678" -> "iota"`
    * @returns {string}
    */
    method() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_method(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the `DID` method-specific ID.
    *
    * E.g.
    * - `"did:example:12345678" -> "12345678"`
    * - `"did:iota:main:12345678" -> "main:12345678"`
    * @returns {string}
    */
    methodId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_methodId(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Construct a new {@link DIDUrl} by joining with a relative DID Url string.
    * @param {string} segment
    * @returns {DIDUrl}
    */
    join(segment) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(segment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadid_join(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DIDUrl.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Clones the `DID` into a {@link DIDUrl}.
    * @returns {DIDUrl}
    */
    toUrl() {
        const ret = wasm.iotadid_toUrl(this.__wbg_ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Returns the hex-encoded AliasId with a '0x' prefix, from the DID tag.
    * @returns {string}
    */
    toAliasId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_toAliasId(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Converts the `DID` into a {@link DIDUrl}, consuming it.
    * @returns {DIDUrl}
    */
    intoUrl() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.iotadid_intoUrl(ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Returns the `DID` as a string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {IotaDID}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadid_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {IotaDID}
    */
    clone() {
        const ret = wasm.iotadid_clone(this.__wbg_ptr);
        return IotaDID.__wrap(ret);
    }
}
module.exports.IotaDID = IotaDID;

const IotaDocumentFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_iotadocument_free(ptr >>> 0, 1));
/**
* A DID Document adhering to the IOTA DID method specification.
*
* Note: All methods that involve reading from this class may potentially raise an error
* if the object is being concurrently modified.
*/
class IotaDocument {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IotaDocument.prototype);
        obj.__wbg_ptr = ptr;
        IotaDocumentFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IotaDocumentFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_iotadocument_free(ptr, 0);
    }
    /**
    * Constructs an empty IOTA DID Document with a {@link IotaDID.placeholder} identifier
    * for the given `network`.
    * @param {string} network
    */
    constructor(network) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(network, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadocument_new(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            IotaDocumentFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Constructs an empty DID Document with the given identifier.
    * @param {IotaDID} id
    * @returns {IotaDocument}
    */
    static newWithId(id) {
        _assertClass(id, IotaDID);
        const ret = wasm.iotadocument_newWithId(id.__wbg_ptr);
        return IotaDocument.__wrap(ret);
    }
    /**
    * Returns a copy of the DID Document `id`.
    * @returns {IotaDID}
    */
    id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the list of document controllers.
    *
    * NOTE: controllers are determined by the `state_controller` unlock condition of the output
    * during resolution and are omitted when publishing.
    * @returns {IotaDID[]}
    */
    controller() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_controller(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the controllers of the document.
    *
    * Note: Duplicates will be ignored.
    * Use `null` to remove all controllers.
    * @param {IotaDID[] | null} controller
    */
    setController(controller) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_setController(retptr, this.__wbg_ptr, addBorrowedObject(controller));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a copy of the document's `alsoKnownAs` set.
    * @returns {Array<string>}
    */
    alsoKnownAs() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_alsoKnownAs(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the `alsoKnownAs` property in the DID document.
    * @param {string | string[] | null} urls
    */
    setAlsoKnownAs(urls) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_setAlsoKnownAs(retptr, this.__wbg_ptr, addBorrowedObject(urls));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a copy of the custom DID Document properties.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a custom property in the DID Document.
    * If the value is set to `null`, the custom property will be removed.
    *
    * ### WARNING
    *
    * This method can overwrite existing properties like `id` and result in an invalid document.
    * @param {string} key
    * @param {any} value
    */
    setPropertyUnchecked(key, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadocument_setPropertyUnchecked(retptr, this.__wbg_ptr, ptr0, len0, addBorrowedObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Return a set of all {@link Service} in the document.
    * @returns {Service[]}
    */
    service() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_service(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Add a new {@link Service} to the document.
    *
    * Returns `true` if the service was added.
    * @param {Service} service
    */
    insertService(service) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(service, Service);
            wasm.iotadocument_insertService(retptr, this.__wbg_ptr, service.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Remove a {@link Service} identified by the given {@link DIDUrl} from the document.
    *
    * Returns `true` if a service was removed.
    * @param {DIDUrl} did
    * @returns {Service | undefined}
    */
    removeService(did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, DIDUrl);
            wasm.iotadocument_removeService(retptr, this.__wbg_ptr, did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the first {@link Service} with an `id` property matching the provided `query`,
    * if present.
    * @param {DIDUrl | string} query
    * @returns {Service | undefined}
    */
    resolveService(query) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_resolveService(retptr, this.__wbg_ptr, addBorrowedObject(query));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a list of all {@link VerificationMethod} in the DID Document,
    * whose verification relationship matches `scope`.
    *
    * If `scope` is not set, a list over the **embedded** methods is returned.
    * @param {MethodScope | undefined} [scope]
    * @returns {VerificationMethod[]}
    */
    methods(scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_methods(retptr, this.__wbg_ptr, isLikeNone(scope) ? 0 : addHeapObject(scope));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds a new `method` to the document in the given `scope`.
    * @param {VerificationMethod} method
    * @param {MethodScope} scope
    */
    insertMethod(method, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(method, VerificationMethod);
            _assertClass(scope, MethodScope);
            wasm.iotadocument_insertMethod(retptr, this.__wbg_ptr, method.__wbg_ptr, scope.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Removes all references to the specified Verification Method.
    * @param {DIDUrl} did
    * @returns {VerificationMethod | undefined}
    */
    removeMethod(did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, DIDUrl);
            wasm.iotadocument_removeMethod(retptr, this.__wbg_ptr, did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the first verification method with an `id` property
    * matching the provided `query` and the verification relationship
    * specified by `scope`, if present.
    * @param {DIDUrl | string} query
    * @param {MethodScope | undefined} [scope]
    * @returns {VerificationMethod | undefined}
    */
    resolveMethod(query, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_resolveMethod(retptr, this.__wbg_ptr, addBorrowedObject(query), isLikeNone(scope) ? 0 : addHeapObject(scope));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Attaches the relationship to the given method, if the method exists.
    *
    * Note: The method needs to be in the set of verification methods,
    * so it cannot be an embedded one.
    * @param {DIDUrl} didUrl
    * @param {MethodRelationship} relationship
    * @returns {boolean}
    */
    attachMethodRelationship(didUrl, relationship) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(didUrl, DIDUrl);
            wasm.iotadocument_attachMethodRelationship(retptr, this.__wbg_ptr, didUrl.__wbg_ptr, relationship);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Detaches the given relationship from the given method, if the method exists.
    * @param {DIDUrl} didUrl
    * @param {MethodRelationship} relationship
    * @returns {boolean}
    */
    detachMethodRelationship(didUrl, relationship) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(didUrl, DIDUrl);
            wasm.iotadocument_detachMethodRelationship(retptr, this.__wbg_ptr, didUrl.__wbg_ptr, relationship);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Decodes and verifies the provided JWS according to the passed `options` and `signatureVerifier`.
    *  If no `signatureVerifier` argument is provided a default verifier will be used that is (only) capable of
    * verifying EdDSA signatures.
    *
    * Regardless of which options are passed the following conditions must be met in order for a verification attempt to
    * take place.
    * - The JWS must be encoded according to the JWS compact serialization.
    * - The `kid` value in the protected header must be an identifier of a verification method in this DID document.
    * @param {Jws} jws
    * @param {JwsVerificationOptions} options
    * @param {IJwsVerifier} signatureVerifier
    * @param {string | undefined} [detachedPayload]
    * @returns {DecodedJws}
    */
    verifyJws(jws, options, signatureVerifier, detachedPayload) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(jws, Jws);
            _assertClass(options, JwsVerificationOptions);
            var ptr0 = isLikeNone(detachedPayload) ? 0 : passStringToWasm0(detachedPayload, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.iotadocument_verifyJws(retptr, this.__wbg_ptr, jws.__wbg_ptr, options.__wbg_ptr, addHeapObject(signatureVerifier), ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJws.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes the document for inclusion in an Alias Output's state metadata
    * with the default {@link StateMetadataEncoding}.
    * @returns {Uint8Array}
    */
    pack() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_pack(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes the document for inclusion in an Alias Output's state metadata.
    * @param {StateMetadataEncoding} encoding
    * @returns {Uint8Array}
    */
    packWithEncoding(encoding) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_packWithEncoding(retptr, this.__wbg_ptr, encoding);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes the document from an Alias Output.
    *
    * If `allowEmpty` is true, this will return an empty DID document marked as `deactivated`
    * if `stateMetadata` is empty.
    *
    * The `tokenSupply` must be equal to the token supply of the network the DID is associated with.
    *
    * NOTE: `did` is required since it is omitted from the serialized DID Document and
    * cannot be inferred from the state metadata. It also indicates the network, which is not
    * encoded in the `AliasId` alone.
    * @param {IotaDID} did
    * @param {AliasOutputBuilderParams} aliasOutput
    * @param {boolean} allowEmpty
    * @returns {IotaDocument}
    */
    static unpackFromOutput(did, aliasOutput, allowEmpty) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, IotaDID);
            wasm.iotadocument_unpackFromOutput(retptr, did.__wbg_ptr, addHeapObject(aliasOutput), allowEmpty);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns all DID documents of the Alias Outputs contained in the block's transaction payload
    * outputs, if any.
    *
    * Errors if any Alias Output does not contain a valid or empty DID Document.
    * @param {string} network
    * @param {Block} block
    * @returns {IotaDocument[]}
    */
    static unpackFromBlock(network, block) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(network, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadocument_unpackFromBlock(retptr, ptr0, len0, addBorrowedObject(block));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a copy of the metadata associated with this document.
    *
    * NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
    * `metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.
    * @returns {IotaDocumentMetadata}
    */
    metadata() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadata(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDocumentMetadata.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the timestamp of when the DID document was created.
    * @returns {Timestamp | undefined}
    */
    metadataCreated() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadataCreated(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Timestamp.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the timestamp of when the DID document was created.
    * @param {Timestamp | undefined} timestamp
    */
    setMetadataCreated(timestamp) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_setMetadataCreated(retptr, this.__wbg_ptr, addHeapObject(timestamp));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the timestamp of the last DID document update.
    * @returns {Timestamp | undefined}
    */
    metadataUpdated() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadataUpdated(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0 ? undefined : Timestamp.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the timestamp of the last DID document update.
    * @param {Timestamp | undefined} timestamp
    */
    setMetadataUpdated(timestamp) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_setMetadataUpdated(retptr, this.__wbg_ptr, addHeapObject(timestamp));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the deactivated status of the DID document.
    * @returns {boolean | undefined}
    */
    metadataDeactivated() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadataDeactivated(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 === 0xFFFFFF ? undefined : r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the deactivated status of the DID document.
    * @param {boolean | undefined} [deactivated]
    */
    setMetadataDeactivated(deactivated) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_setMetadataDeactivated(retptr, this.__wbg_ptr, isLikeNone(deactivated) ? 0xFFFFFF : deactivated ? 1 : 0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the Bech32-encoded state controller address, if present.
    * @returns {string | undefined}
    */
    metadataStateControllerAddress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadataStateControllerAddress(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the Bech32-encoded governor address, if present.
    * @returns {string | undefined}
    */
    metadataGovernorAddress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_metadataGovernorAddress(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a custom property in the document metadata.
    * If the value is set to `null`, the custom property will be removed.
    * @param {string} key
    * @param {any} value
    */
    setMetadataPropertyUnchecked(key, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.iotadocument_setMetadataPropertyUnchecked(retptr, this.__wbg_ptr, ptr0, len0, addBorrowedObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * If the document has a {@link RevocationBitmap} service identified by `serviceQuery`,
    * revoke all specified `indices`.
    * @param {DIDUrl | string} serviceQuery
    * @param {number | number[]} indices
    */
    revokeCredentials(serviceQuery, indices) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_revokeCredentials(retptr, this.__wbg_ptr, addBorrowedObject(serviceQuery), addHeapObject(indices));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * If the document has a {@link RevocationBitmap} service identified by `serviceQuery`,
    * unrevoke all specified `indices`.
    * @param {DIDUrl | string} serviceQuery
    * @param {number | number[]} indices
    */
    unrevokeCredentials(serviceQuery, indices) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_unrevokeCredentials(retptr, this.__wbg_ptr, addBorrowedObject(serviceQuery), addHeapObject(indices));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Returns a deep clone of the {@link IotaDocument}.
    * @returns {IotaDocument}
    */
    clone() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_clone(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * ### Warning
    * This is for internal use only. Do not rely on or call this method.
    * @returns {IotaDocument}
    */
    _shallowCloneInternal() {
        const ret = wasm.iotadocument__shallowCloneInternal(this.__wbg_ptr);
        return IotaDocument.__wrap(ret);
    }
    /**
    * ### Warning
    * This is for internal use only. Do not rely on or call this method.
    * @returns {number}
    */
    _strongCountInternal() {
        const ret = wasm.iotadocument__strongCountInternal(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Serializes to a plain JS representation.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a plain JS representation.
    * @param {any} json
    * @returns {IotaDocument}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Transforms the {@link IotaDocument} to its {@link CoreDocument} representation.
    * @returns {CoreDocument}
    */
    toCoreDocument() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocument_toCoreDocument(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDocument.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Generate new key material in the given `storage` and insert a new verification method with the corresponding
    * public key material into the DID document.
    *
    * - If no fragment is given the `kid` of the generated JWK is used, if it is set, otherwise an error is returned.
    * - The `keyType` must be compatible with the given `storage`. `Storage`s are expected to export key type constants
    * for that use case.
    *
    * The fragment of the generated method is returned.
    * @param {Storage} storage
    * @param {string} keyType
    * @param {JwsAlgorithm} alg
    * @param {string | undefined} fragment
    * @param {MethodScope} scope
    * @returns {Promise<string>}
    */
    generateMethod(storage, keyType, alg, fragment, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(keyType, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(fragment) ? 0 : passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            _assertClass(scope, MethodScope);
            var ptr2 = scope.__destroy_into_raw();
            wasm.iotadocument_generateMethod(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, addHeapObject(alg), ptr1, len1, ptr2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Remove the method identified by the given fragment from the document and delete the corresponding key material in
    * the given `storage`.
    * @param {Storage} storage
    * @param {DIDUrl} id
    * @returns {Promise<void>}
    */
    purgeMethod(storage, id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            _assertClass(id, DIDUrl);
            wasm.iotadocument_purgeMethod(retptr, this.__wbg_ptr, storage.__wbg_ptr, id.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
    * material in the verification method identified by the given `fragment.
    *
    * Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
    * See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).
    *
    * @deprecated Use `createJws()` instead.
    * @param {Storage} storage
    * @param {string} fragment
    * @param {string} payload
    * @param {JwsSignatureOptions} options
    * @returns {Promise<Jws>}
    */
    createJwt(storage, fragment, payload, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(payload, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwsSignatureOptions);
            wasm.iotadocument_createJwt(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, ptr1, len1, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
    * material in the verification method identified by the given `fragment.
    *
    * Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
    * See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).
    * @param {Storage} storage
    * @param {string} fragment
    * @param {string} payload
    * @param {JwsSignatureOptions} options
    * @returns {Promise<Jws>}
    */
    createJws(storage, fragment, payload, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(payload, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwsSignatureOptions);
            wasm.iotadocument_createJws(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, ptr1, len1, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Produces a JWS where the payload is produced from the given `credential`
    * in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
    *
    * Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
    * of the method identified by `fragment` and the JWS signature will be produced by the corresponding
    * private key backed by the `storage` in accordance with the passed `options`.
    *
    * The `custom_claims` can be used to set additional claims on the resulting JWT.
    * @param {Storage} storage
    * @param {string} fragment
    * @param {Credential} credential
    * @param {JwsSignatureOptions} options
    * @param {Record<string, any> | undefined} [custom_claims]
    * @returns {Promise<Jwt>}
    */
    createCredentialJwt(storage, fragment, credential, options, custom_claims) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(credential, Credential);
            _assertClass(options, JwsSignatureOptions);
            wasm.iotadocument_createCredentialJwt(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, credential.__wbg_ptr, options.__wbg_ptr, isLikeNone(custom_claims) ? 0 : addHeapObject(custom_claims));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Produces a JWT where the payload is produced from the given presentation.
    * in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).
    *
    * Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
    * of the method identified by `fragment` and the JWS signature will be produced by the corresponding
    * private key backed by the `storage` in accordance with the passed `options`.
    * @param {Storage} storage
    * @param {string} fragment
    * @param {Presentation} presentation
    * @param {JwsSignatureOptions} signature_options
    * @param {JwtPresentationOptions} presentation_options
    * @returns {Promise<Jwt>}
    */
    createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(presentation, Presentation);
            _assertClass(signature_options, JwsSignatureOptions);
            _assertClass(presentation_options, JwtPresentationOptions);
            wasm.iotadocument_createPresentationJwt(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, presentation.__wbg_ptr, signature_options.__wbg_ptr, presentation_options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Storage} storage
    * @param {ProofAlgorithm} alg
    * @param {string | undefined} fragment
    * @param {MethodScope} scope
    * @returns {Promise<string>}
    */
    generateMethodJwp(storage, alg, fragment, scope) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            var ptr0 = isLikeNone(fragment) ? 0 : passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _assertClass(scope, MethodScope);
            var ptr1 = scope.__destroy_into_raw();
            wasm.iotadocument_generateMethodJwp(retptr, this.__wbg_ptr, storage.__wbg_ptr, alg, ptr0, len0, ptr1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Storage} storage
    * @param {string} fragment
    * @param {JptClaims} jpt_claims
    * @param {JwpCredentialOptions} options
    * @returns {Promise<string>}
    */
    createIssuedJwp(storage, fragment, jpt_claims, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(storage, Storage);
            const ptr0 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(options, JwpCredentialOptions);
            var ptr1 = options.__destroy_into_raw();
            wasm.iotadocument_createIssuedJwp(retptr, this.__wbg_ptr, storage.__wbg_ptr, ptr0, len0, addHeapObject(jpt_claims), ptr1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {SelectiveDisclosurePresentation} presentation
    * @param {string} method_id
    * @param {JwpPresentationOptions} options
    * @returns {Promise<string>}
    */
    createPresentedJwp(presentation, method_id, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation, SelectiveDisclosurePresentation);
            var ptr0 = presentation.__destroy_into_raw();
            const ptr1 = passStringToWasm0(method_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwpPresentationOptions);
            var ptr2 = options.__destroy_into_raw();
            wasm.iotadocument_createPresentedJwp(retptr, this.__wbg_ptr, ptr0, ptr1, len1, ptr2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Credential} credential
    * @param {Storage} storage
    * @param {string} fragment
    * @param {JwpCredentialOptions} options
    * @param {Map<string, any> | undefined} [custom_claims]
    * @returns {Promise<Jpt>}
    */
    createCredentialJpt(credential, storage, fragment, options, custom_claims) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            var ptr0 = credential.__destroy_into_raw();
            _assertClass(storage, Storage);
            const ptr1 = passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwpCredentialOptions);
            var ptr2 = options.__destroy_into_raw();
            wasm.iotadocument_createCredentialJpt(retptr, this.__wbg_ptr, ptr0, storage.__wbg_ptr, ptr1, len1, ptr2, isLikeNone(custom_claims) ? 0 : addHeapObject(custom_claims));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {SelectiveDisclosurePresentation} presentation
    * @param {string} method_id
    * @param {JwpPresentationOptions} options
    * @returns {Promise<Jpt>}
    */
    createPresentationJpt(presentation, method_id, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation, SelectiveDisclosurePresentation);
            var ptr0 = presentation.__destroy_into_raw();
            const ptr1 = passStringToWasm0(method_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            _assertClass(options, JwpPresentationOptions);
            var ptr2 = options.__destroy_into_raw();
            wasm.iotadocument_createPresentationJpt(retptr, this.__wbg_ptr, ptr0, ptr1, len1, ptr2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.IotaDocument = IotaDocument;

const IotaDocumentMetadataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_iotadocumentmetadata_free(ptr >>> 0, 1));
/**
* Additional attributes related to an IOTA DID Document.
*/
class IotaDocumentMetadata {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IotaDocumentMetadata.prototype);
        obj.__wbg_ptr = ptr;
        IotaDocumentMetadataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IotaDocumentMetadataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_iotadocumentmetadata_free(ptr, 0);
    }
    /**
    * Returns a copy of the timestamp of when the DID document was created.
    * @returns {Timestamp | undefined}
    */
    created() {
        const ret = wasm.iotadocumentmetadata_created(this.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * Returns a copy of the timestamp of the last DID document update.
    * @returns {Timestamp | undefined}
    */
    updated() {
        const ret = wasm.iotadocumentmetadata_updated(this.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * Returns a copy of the deactivated status of the DID document.
    * @returns {boolean | undefined}
    */
    deactivated() {
        const ret = wasm.iotadocumentmetadata_deactivated(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * Returns a copy of the Bech32-encoded state controller address, if present.
    * @returns {string | undefined}
    */
    stateControllerAddress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocumentmetadata_stateControllerAddress(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the Bech32-encoded governor address, if present.
    * @returns {string | undefined}
    */
    governorAddress() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocumentmetadata_governorAddress(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the custom metadata properties.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocumentmetadata_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocumentmetadata_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {IotaDocumentMetadata}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.iotadocumentmetadata_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return IotaDocumentMetadata.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {IotaDocumentMetadata}
    */
    clone() {
        const ret = wasm.iotadocumentmetadata_clone(this.__wbg_ptr);
        return IotaDocumentMetadata.__wrap(ret);
    }
}
module.exports.IotaDocumentMetadata = IotaDocumentMetadata;

const IotaIdentityClientExtFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_iotaidentityclientext_free(ptr >>> 0, 1));
/**
* An extension interface that provides helper functions for publication
* and resolution of DID documents in Alias Outputs.
*/
class IotaIdentityClientExt {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IotaIdentityClientExtFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_iotaidentityclientext_free(ptr, 0);
    }
    /**
    * Create a DID with a new Alias Output containing the given `document`.
    *
    * The `address` will be set as the state controller and governor unlock conditions.
    * The minimum required token deposit amount will be set according to the given
    * `rent_structure`, which will be fetched from the node if not provided.
    * The returned Alias Output can be further customised before publication, if desired.
    *
    * NOTE: this does *not* publish the Alias Output.
    * @param {IIotaIdentityClient} client
    * @param {Address} address
    * @param {IotaDocument} document
    * @param {IRent | undefined} [rentStructure]
    * @returns {Promise<AliasOutputBuilderParams>}
    */
    static newDidOutput(client, address, document, rentStructure) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(document, IotaDocument);
            wasm.iotaidentityclientext_newDidOutput(retptr, addHeapObject(client), addHeapObject(address), document.__wbg_ptr, isLikeNone(rentStructure) ? 0 : addHeapObject(rentStructure));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Fetches the associated Alias Output and updates it with `document` in its state metadata.
    * The storage deposit on the output is left unchanged. If the size of the document increased,
    * the amount should be increased manually.
    *
    * NOTE: this does *not* publish the updated Alias Output.
    * @param {IIotaIdentityClient} client
    * @param {IotaDocument} document
    * @returns {Promise<AliasOutputBuilderParams>}
    */
    static updateDidOutput(client, document) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(document, IotaDocument);
            wasm.iotaidentityclientext_updateDidOutput(retptr, addHeapObject(client), document.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Removes the DID document from the state metadata of its Alias Output,
    * effectively deactivating it. The storage deposit on the output is left unchanged,
    * and should be reallocated manually.
    *
    * Deactivating does not destroy the output. Hence, it can be re-activated by publishing
    * an update containing a DID document.
    *
    * NOTE: this does *not* publish the updated Alias Output.
    * @param {IIotaIdentityClient} client
    * @param {IotaDID} did
    * @returns {Promise<AliasOutputBuilderParams>}
    */
    static deactivateDidOutput(client, did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, IotaDID);
            wasm.iotaidentityclientext_deactivateDidOutput(retptr, addHeapObject(client), did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Resolve a {@link IotaDocument}. Returns an empty, deactivated document if the state metadata
    * of the Alias Output is empty.
    * @param {IIotaIdentityClient} client
    * @param {IotaDID} did
    * @returns {Promise<IotaDocument>}
    */
    static resolveDid(client, did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, IotaDID);
            wasm.iotaidentityclientext_resolveDid(retptr, addHeapObject(client), did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Fetches the `IAliasOutput` associated with the given DID.
    * @param {IIotaIdentityClient} client
    * @param {IotaDID} did
    * @returns {Promise<AliasOutputBuilderParams>}
    */
    static resolveDidOutput(client, did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(did, IotaDID);
            wasm.iotaidentityclientext_resolveDidOutput(retptr, addHeapObject(client), did.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.IotaIdentityClientExt = IotaIdentityClientExt;

const IssuerProtectedHeaderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_issuerprotectedheader_free(ptr >>> 0, 1));
/**
*/
class IssuerProtectedHeader {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(IssuerProtectedHeader.prototype);
        obj.__wbg_ptr = ptr;
        IssuerProtectedHeaderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
            typ: this.typ,
            alg: this.alg,
            kid: this.kid,
            cid: this.cid,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IssuerProtectedHeaderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_issuerprotectedheader_free(ptr, 0);
    }
    /**
    * JWP type (JPT).
    * @returns {string | undefined}
    */
    get typ() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_issuerprotectedheader_typ(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * JWP type (JPT).
    * @param {string | undefined} [arg0]
    */
    set typ(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_issuerprotectedheader_typ(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Algorithm used for the JWP.
    * @returns {ProofAlgorithm}
    */
    get alg() {
        const ret = wasm.__wbg_get_issuerprotectedheader_alg(this.__wbg_ptr);
        return ret;
    }
    /**
    * Algorithm used for the JWP.
    * @param {ProofAlgorithm} arg0
    */
    set alg(arg0) {
        wasm.__wbg_set_issuerprotectedheader_alg(this.__wbg_ptr, arg0);
    }
    /**
    * ID for the key used for the JWP.
    * @returns {string | undefined}
    */
    get kid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_issuerprotectedheader_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * ID for the key used for the JWP.
    * @param {string | undefined} [arg0]
    */
    set kid(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_issuerprotectedheader_kid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Not handled for now. Will be used in the future to resolve external claims
    * @returns {string | undefined}
    */
    get cid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_issuerprotectedheader_cid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Not handled for now. Will be used in the future to resolve external claims
    * @param {string | undefined} [arg0]
    */
    set cid(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_issuerprotectedheader_cid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {(string)[]}
    */
    claims() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.issuerprotectedheader_claims(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.IssuerProtectedHeader = IssuerProtectedHeader;

const JptFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jpt_free(ptr >>> 0, 1));
/**
* A JSON Proof Token (JPT).
*/
class Jpt {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Jpt.prototype);
        obj.__wbg_ptr = ptr;
        JptFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jpt_free(ptr, 0);
    }
    /**
    * Creates a new {@link Jpt}.
    * @param {string} jpt_string
    */
    constructor(jpt_string) {
        const ptr0 = passStringToWasm0(jpt_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jpt_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        JptFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jpt_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Deep clones the object.
    * @returns {Jpt}
    */
    clone() {
        const ret = wasm.jpt_clone(this.__wbg_ptr);
        return Jpt.__wrap(ret);
    }
}
module.exports.Jpt = Jpt;

const JptCredentialValidationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptcredentialvalidationoptions_free(ptr >>> 0, 1));
/**
* Options to declare validation criteria for {@link Jpt}.
*/
class JptCredentialValidationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JptCredentialValidationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JptCredentialValidationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptCredentialValidationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptcredentialvalidationoptions_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {JptCredentialValidationOptions}
    */
    clone() {
        const ret = wasm.jptcredentialvalidationoptions_clone(this.__wbg_ptr);
        return JptCredentialValidationOptions.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptcredentialvalidationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JptCredentialValidationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptcredentialvalidationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JptCredentialValidationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Creates a new default istance.
    * @param {IJptCredentialValidationOptions | undefined} [opts]
    */
    constructor(opts) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptcredentialvalidationoptions_new(retptr, isLikeNone(opts) ? 0 : addHeapObject(opts));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JptCredentialValidationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JptCredentialValidationOptions = JptCredentialValidationOptions;

const JptCredentialValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptcredentialvalidator_free(ptr >>> 0, 1));
/**
*/
class JptCredentialValidator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptCredentialValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptcredentialvalidator_free(ptr, 0);
    }
    /**
    * @param {Jpt} credential_jpt
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {JptCredentialValidationOptions} options
    * @param {FailFast} fail_fast
    * @returns {DecodedJptCredential}
    */
    static validate(credential_jpt, issuer, options, fail_fast) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential_jpt, Jpt);
            _assertClass(options, JptCredentialValidationOptions);
            wasm.jptcredentialvalidator_validate(retptr, credential_jpt.__wbg_ptr, addBorrowedObject(issuer), options.__wbg_ptr, fail_fast);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJptCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.JptCredentialValidator = JptCredentialValidator;

const JptCredentialValidatorUtilsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptcredentialvalidatorutils_free(ptr >>> 0, 1));
/**
* Utility functions for validating JPT credentials.
*/
class JptCredentialValidatorUtils {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptCredentialValidatorUtilsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptcredentialvalidatorutils_free(ptr, 0);
    }
    /**
    */
    constructor() {
        const ret = wasm.jptcredentialvalidatorutils_new();
        this.__wbg_ptr = ret >>> 0;
        JptCredentialValidatorUtilsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Utility for extracting the issuer field of a {@link Credential} as a DID.
    * # Errors
    * Fails if the issuer field is not a valid DID.
    * @param {Credential} credential
    * @returns {CoreDID}
    */
    static extractIssuer(credential) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            wasm.jptcredentialvalidatorutils_extractIssuer(retptr, credential.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Utility for extracting the issuer field of a credential in JPT representation as DID.
    * # Errors
    * If the JPT decoding fails or the issuer field is not a valid DID.
    * @param {Jpt} credential
    * @returns {CoreDID}
    */
    static extractIssuerFromIssuedJpt(credential) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Jpt);
            wasm.jptcredentialvalidatorutils_extractIssuerFromIssuedJpt(retptr, credential.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Credential} credential
    * @param {Timestamp | undefined} validity_timeframe
    * @param {StatusCheck} status_check
    */
    static checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            let ptr0 = 0;
            if (!isLikeNone(validity_timeframe)) {
                _assertClass(validity_timeframe, Timestamp);
                ptr0 = validity_timeframe.__destroy_into_raw();
            }
            wasm.jptcredentialvalidatorutils_checkTimeframesWithValidityTimeframe2024(retptr, credential.__wbg_ptr, ptr0, status_check);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Checks whether the credential status has been revoked.
    *
    * Only supports `RevocationTimeframe2024`.
    * @param {Credential} credential
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {StatusCheck} status_check
    */
    static checkRevocationWithValidityTimeframe2024(credential, issuer, status_check) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            wasm.jptcredentialvalidatorutils_checkRevocationWithValidityTimeframe2024(retptr, credential.__wbg_ptr, addBorrowedObject(issuer), status_check);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Checks whether the credential status has been revoked or the timeframe interval is INVALID
    *
    * Only supports `RevocationTimeframe2024`.
    * @param {Credential} credential
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {Timestamp | undefined} validity_timeframe
    * @param {StatusCheck} status_check
    */
    static checkTimeframesAndRevocationWithValidityTimeframe2024(credential, issuer, validity_timeframe, status_check) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            let ptr0 = 0;
            if (!isLikeNone(validity_timeframe)) {
                _assertClass(validity_timeframe, Timestamp);
                ptr0 = validity_timeframe.__destroy_into_raw();
            }
            wasm.jptcredentialvalidatorutils_checkTimeframesAndRevocationWithValidityTimeframe2024(retptr, credential.__wbg_ptr, addBorrowedObject(issuer), ptr0, status_check);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.JptCredentialValidatorUtils = JptCredentialValidatorUtils;

const JptPresentationValidationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptpresentationvalidationoptions_free(ptr >>> 0, 1));
/**
* Options to declare validation criteria for a {@link Jpt} presentation.
*/
class JptPresentationValidationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JptPresentationValidationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JptPresentationValidationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptPresentationValidationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptpresentationvalidationoptions_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {JptPresentationValidationOptions}
    */
    clone() {
        const ret = wasm.jptpresentationvalidationoptions_clone(this.__wbg_ptr);
        return JptPresentationValidationOptions.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptpresentationvalidationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JptPresentationValidationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptpresentationvalidationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JptPresentationValidationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {IJptPresentationValidationOptions | undefined} [opts]
    */
    constructor(opts) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jptpresentationvalidationoptions_new(retptr, isLikeNone(opts) ? 0 : addHeapObject(opts));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JptPresentationValidationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JptPresentationValidationOptions = JptPresentationValidationOptions;

const JptPresentationValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptpresentationvalidator_free(ptr >>> 0, 1));
/**
*/
class JptPresentationValidator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptPresentationValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptpresentationvalidator_free(ptr, 0);
    }
    /**
    * Decodes and validates a Presented {@link Credential} issued as a JPT (JWP Presented Form). A
    * {@link DecodedJptPresentation} is returned upon success.
    *
    * The following properties are validated according to `options`:
    * - the holder's proof on the JWP,
    * - the expiration date,
    * - the issuance date,
    * - the semantic structure.
    * @param {Jpt} presentation_jpt
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {JptPresentationValidationOptions} options
    * @param {FailFast} fail_fast
    * @returns {DecodedJptPresentation}
    */
    static validate(presentation_jpt, issuer, options, fail_fast) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation_jpt, Jpt);
            _assertClass(options, JptPresentationValidationOptions);
            wasm.jptpresentationvalidator_validate(retptr, presentation_jpt.__wbg_ptr, addBorrowedObject(issuer), options.__wbg_ptr, fail_fast);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJptPresentation.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.JptPresentationValidator = JptPresentationValidator;

const JptPresentationValidatorUtilsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jptpresentationvalidatorutils_free(ptr >>> 0, 1));
/**
* Utility functions for verifying JPT presentations.
*/
class JptPresentationValidatorUtils {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JptPresentationValidatorUtilsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jptpresentationvalidatorutils_free(ptr, 0);
    }
    /**
    * Utility for extracting the issuer field of a credential in JPT representation as DID.
    * # Errors
    * If the JPT decoding fails or the issuer field is not a valid DID.
    * @param {Jpt} presentation
    * @returns {CoreDID}
    */
    static extractIssuerFromPresentedJpt(presentation) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation, Jpt);
            wasm.jptpresentationvalidatorutils_extractIssuerFromPresentedJpt(retptr, presentation.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Check timeframe interval in credentialStatus with `RevocationTimeframeStatus`.
    * @param {Credential} credential
    * @param {Timestamp | undefined} validity_timeframe
    * @param {StatusCheck} status_check
    */
    static checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            let ptr0 = 0;
            if (!isLikeNone(validity_timeframe)) {
                _assertClass(validity_timeframe, Timestamp);
                ptr0 = validity_timeframe.__destroy_into_raw();
            }
            wasm.jptpresentationvalidatorutils_checkTimeframesWithValidityTimeframe2024(retptr, credential.__wbg_ptr, ptr0, status_check);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JptPresentationValidatorUtils = JptPresentationValidatorUtils;

const JwkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwk_free(ptr >>> 0, 1));
/**
*/
class Jwk {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Jwk.prototype);
        obj.__wbg_ptr = ptr;
        JwkFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwk_free(ptr, 0);
    }
    /**
    * @param {IJwkParams} jwk
    */
    constructor(jwk) {
        const ret = wasm.jwk_new(addHeapObject(jwk));
        this.__wbg_ptr = ret >>> 0;
        JwkFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Returns the value for the key type parameter (kty).
    * @returns {JwkType}
    */
    kty() {
        const ret = wasm.jwk_kty(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the value for the use property (use).
    * @returns {JwkUse | undefined}
    */
    use() {
        const ret = wasm.jwk_use(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<JwkOperation>}
    */
    keyOps() {
        const ret = wasm.jwk_keyOps(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the value for the algorithm property (alg).
    * @returns {JwsAlgorithm | undefined}
    */
    alg() {
        const ret = wasm.jwk_alg(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the value of the key ID property (kid).
    * @returns {string | undefined}
    */
    kid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the X.509 URL property (x5u).
    * @returns {string | undefined}
    */
    x5u() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_x5u(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the X.509 certificate chain property (x5c).
    * @returns {Array<string>}
    */
    x5c() {
        const ret = wasm.jwk_x5c(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the value of the X.509 certificate SHA-1 thumbprint property (x5t).
    * @returns {string | undefined}
    */
    x5t() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_x5t(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the X.509 certificate SHA-256 thumbprint property (x5t#S256).
    * @returns {string | undefined}
    */
    x5t256() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_x5t256(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * If this JWK is of kty EC, returns those parameters.
    * @returns {JwkParamsEc | undefined}
    */
    paramsEc() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_paramsEc(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * If this JWK is of kty OKP, returns those parameters.
    * @returns {JwkParamsOkp | undefined}
    */
    paramsOkp() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_paramsOkp(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * If this JWK is of kty OCT, returns those parameters.
    * @returns {JwkParamsOct | undefined}
    */
    paramsOct() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_paramsOct(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * If this JWK is of kty RSA, returns those parameters.
    * @returns {JwkParamsRsa | undefined}
    */
    paramsRsa() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_paramsRsa(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a clone of the {@link Jwk} with _all_ private key components unset.
    * Nothing is returned when `kty = oct` as this key type is not considered public by this library.
    * @returns {Jwk | undefined}
    */
    toPublic() {
        const ret = wasm.jwk_toPublic(this.__wbg_ptr);
        return ret === 0 ? undefined : Jwk.__wrap(ret);
    }
    /**
    * Returns `true` if _all_ private key components of the key are unset, `false` otherwise.
    * @returns {boolean}
    */
    isPublic() {
        const ret = wasm.jwk_isPublic(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Returns `true` if _all_ private key components of the key are set, `false` otherwise.
    * @returns {boolean}
    */
    isPrivate() {
        const ret = wasm.jwk_isPrivate(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Jwk}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwk_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Jwk.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Jwk}
    */
    clone() {
        const ret = wasm.jwk_clone(this.__wbg_ptr);
        return Jwk.__wrap(ret);
    }
}
module.exports.Jwk = Jwk;

const JwkGenOutputFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwkgenoutput_free(ptr >>> 0, 1));
/**
* The result of a key generation in `JwkStorage`.
*/
class JwkGenOutput {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwkGenOutput.prototype);
        obj.__wbg_ptr = ptr;
        JwkGenOutputFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwkGenOutputFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwkgenoutput_free(ptr, 0);
    }
    /**
    * @param {string} key_id
    * @param {Jwk} jwk
    */
    constructor(key_id, jwk) {
        const ptr0 = passStringToWasm0(key_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(jwk, Jwk);
        const ret = wasm.jwkgenoutput_new(ptr0, len0, jwk.__wbg_ptr);
        this.__wbg_ptr = ret >>> 0;
        JwkGenOutputFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Returns the generated public {@link Jwk}.
    * @returns {Jwk}
    */
    jwk() {
        const ret = wasm.jwkgenoutput_jwk(this.__wbg_ptr);
        return Jwk.__wrap(ret);
    }
    /**
    * Returns the key id of the generated {@link Jwk}.
    * @returns {string}
    */
    keyId() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwkgenoutput_keyId(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwkgenoutput_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwkGenOutput}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwkgenoutput_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwkGenOutput.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwkGenOutput}
    */
    clone() {
        const ret = wasm.jwkgenoutput_clone(this.__wbg_ptr);
        return JwkGenOutput.__wrap(ret);
    }
}
module.exports.JwkGenOutput = JwkGenOutput;

const JwkStorageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwkstorage_free(ptr >>> 0, 1));

class JwkStorage {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwkStorageFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwkstorage_free(ptr, 0);
    }
    /**
    * Generates a new BBS+ keypair.
    * @param {ProofAlgorithm} alg
    * @returns {Promise<JwkGenOutput>}
    */
    generateBBS(alg) {
        const ret = wasm.jwkstorage_generateBBS(this.__wbg_ptr, alg);
        return takeObject(ret);
    }
    /**
    * @param {string} key_id
    * @param {(Uint8Array)[]} data
    * @param {Jwk} public_key
    * @param {Uint8Array | undefined} [header]
    * @returns {Promise<Uint8Array>}
    */
    signBBS(key_id, data, public_key, header) {
        const ptr0 = passStringToWasm0(key_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(data, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        _assertClass(public_key, Jwk);
        var ptr2 = public_key.__destroy_into_raw();
        var ptr3 = isLikeNone(header) ? 0 : passArray8ToWasm0(header, wasm.__wbindgen_malloc);
        var len3 = WASM_VECTOR_LEN;
        const ret = wasm.jwkstorage_signBBS(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, ptr3, len3);
        return takeObject(ret);
    }
    /**
    * @param {string} key_id
    * @param {Jwk} public_key
    * @param {Uint8Array} signature
    * @param {ProofUpdateCtx} ctx
    * @returns {Promise<Uint8Array>}
    */
    updateBBSSignature(key_id, public_key, signature, ctx) {
        const ptr0 = passStringToWasm0(key_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(public_key, Jwk);
        const ptr1 = passArray8ToWasm0(signature, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        _assertClass(ctx, ProofUpdateCtx);
        var ptr2 = ctx.__destroy_into_raw();
        const ret = wasm.jwkstorage_updateBBSSignature(this.__wbg_ptr, ptr0, len0, public_key.__wbg_ptr, ptr1, len1, ptr2);
        return takeObject(ret);
    }
}
module.exports.JwkStorage = JwkStorage;

const JwpCredentialOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwpcredentialoptions_free(ptr >>> 0, 1));
/**
*/
class JwpCredentialOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwpCredentialOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwpCredentialOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
            kid: this.kid,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwpCredentialOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwpcredentialoptions_free(ptr, 0);
    }
    /**
    * @returns {string | undefined}
    */
    get kid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_jwpcredentialoptions_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {string | undefined} [arg0]
    */
    set kid(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jwpcredentialoptions_kid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    */
    constructor() {
        const ret = wasm.jwpcredentialoptions_new();
        this.__wbg_ptr = ret >>> 0;
        JwpCredentialOptionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * @param {any} value
    * @returns {JwpCredentialOptions}
    */
    static fromJSON(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpcredentialoptions_fromJSON(retptr, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwpCredentialOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpcredentialoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JwpCredentialOptions = JwpCredentialOptions;

const JwpIssuedFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwpissued_free(ptr >>> 0, 1));
/**
*/
class JwpIssued {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwpIssued.prototype);
        obj.__wbg_ptr = ptr;
        JwpIssuedFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwpIssuedFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwpissued_free(ptr, 0);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpissued_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwpIssued}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpissued_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwpIssued.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwpIssued}
    */
    clone() {
        const ret = wasm.jwpissued_clone(this.__wbg_ptr);
        return JwpIssued.__wrap(ret);
    }
    /**
    * @param {SerializationType} serialization
    * @returns {string}
    */
    encode(serialization) {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpissued_encode(retptr, this.__wbg_ptr, serialization);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * @param {Uint8Array} proof
    */
    setProof(proof) {
        const ptr0 = passArray8ToWasm0(proof, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwpissued_setProof(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {Uint8Array}
    */
    getProof() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpissued_getProof(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Payloads}
    */
    getPayloads() {
        const ret = wasm.jwpissued_getPayloads(this.__wbg_ptr);
        return Payloads.__wrap(ret);
    }
    /**
    * @param {Payloads} payloads
    */
    setPayloads(payloads) {
        _assertClass(payloads, Payloads);
        var ptr0 = payloads.__destroy_into_raw();
        wasm.jwpissued_setPayloads(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {IssuerProtectedHeader}
    */
    getIssuerProtectedHeader() {
        const ret = wasm.jwpissued_getIssuerProtectedHeader(this.__wbg_ptr);
        return IssuerProtectedHeader.__wrap(ret);
    }
}
module.exports.JwpIssued = JwpIssued;

const JwpPresentationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwppresentationoptions_free(ptr >>> 0, 1));
/**
* Options to be set in the JWT claims of a verifiable presentation.
*/
class JwpPresentationOptions {

    toJSON() {
        return {
            audience: this.audience,
            nonce: this.nonce,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwpPresentationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwppresentationoptions_free(ptr, 0);
    }
    /**
    * Sets the audience for presentation (`aud` property in JWP Presentation Header).
    * @returns {string | undefined}
    */
    get audience() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_jwpcredentialoptions_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the audience for presentation (`aud` property in JWP Presentation Header).
    * @param {string | undefined} [arg0]
    */
    set audience(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_jwpcredentialoptions_kid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * The nonce to be placed in the Presentation Protected Header.
    * @returns {string | undefined}
    */
    get nonce() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_issuerprotectedheader_typ(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * The nonce to be placed in the Presentation Protected Header.
    * @param {string | undefined} [arg0]
    */
    set nonce(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_issuerprotectedheader_typ(this.__wbg_ptr, ptr0, len0);
    }
    /**
    */
    constructor() {
        const ret = wasm.jwppresentationoptions_new();
        this.__wbg_ptr = ret >>> 0;
        JwpPresentationOptionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
module.exports.JwpPresentationOptions = JwpPresentationOptions;

const JwpVerificationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwpverificationoptions_free(ptr >>> 0, 1));
/**
*/
class JwpVerificationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwpVerificationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwpVerificationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwpVerificationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwpverificationoptions_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {JwpVerificationOptions}
    */
    clone() {
        const ret = wasm.jwpverificationoptions_clone(this.__wbg_ptr);
        return JwpVerificationOptions.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpverificationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwpVerificationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpverificationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwpVerificationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * @param {IJwpVerificationOptions | undefined} [opts]
    * @returns {JwpVerificationOptions}
    */
    static new(opts) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwpverificationoptions_new(retptr, isLikeNone(opts) ? 0 : addHeapObject(opts));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwpVerificationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JwpVerificationOptions = JwpVerificationOptions;

const JwsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jws_free(ptr >>> 0, 1));
/**
* A wrapper around a JSON Web Signature (JWS).
*/
class Jws {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Jws.prototype);
        obj.__wbg_ptr = ptr;
        JwsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jws_free(ptr, 0);
    }
    /**
    * Creates a new {@link Jws} from the given string.
    * @param {string} jws_string
    */
    constructor(jws_string) {
        const ptr0 = passStringToWasm0(jws_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jws_constructor(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        JwsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Returns a clone of the JWS string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jws_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
module.exports.Jws = Jws;

const JwsHeaderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwsheader_free(ptr >>> 0, 1));
/**
*/
class JwsHeader {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwsHeader.prototype);
        obj.__wbg_ptr = ptr;
        JwsHeaderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwsHeaderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwsheader_free(ptr, 0);
    }
    /**
    * Create a new empty {@link JwsHeader}.
    */
    constructor() {
        const ret = wasm.jwsheader_new();
        this.__wbg_ptr = ret >>> 0;
        JwsHeaderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Returns the value for the algorithm claim (alg).
    * @returns {JwsAlgorithm | undefined}
    */
    alg() {
        const ret = wasm.jwsheader_alg(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Sets a value for the algorithm claim (alg).
    * @param {JwsAlgorithm} value
    */
    setAlg(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_setAlg(retptr, this.__wbg_ptr, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the base64url-encode payload claim (b64).
    * @returns {boolean | undefined}
    */
    b64() {
        const ret = wasm.jwsheader_b64(this.__wbg_ptr);
        return ret === 0xFFFFFF ? undefined : ret !== 0;
    }
    /**
    * Sets a value for the base64url-encode payload claim (b64).
    * @param {boolean} value
    */
    setB64(value) {
        wasm.jwsheader_setB64(this.__wbg_ptr, value);
    }
    /**
    * Additional header parameters.
    * @returns {Record<string, any> | undefined}
    */
    custom() {
        const ret = wasm.jwsheader_custom(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} claim
    * @returns {boolean}
    */
    has(claim) {
        const ptr0 = passStringToWasm0(claim, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jwsheader_has(this.__wbg_ptr, ptr0, len0);
        return ret !== 0;
    }
    /**
    * Returns `true` if none of the fields are set in both `self` and `other`.
    * @param {JwsHeader} other
    * @returns {boolean}
    */
    isDisjoint(other) {
        _assertClass(other, JwsHeader);
        const ret = wasm.jwsheader_isDisjoint(this.__wbg_ptr, other.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Returns the value of the JWK Set URL claim (jku).
    * @returns {string | undefined}
    */
    jku() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_jku(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the JWK Set URL claim (jku).
    * @param {string} value
    */
    setJku(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.jwsheader_setJku(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the JWK claim (jwk).
    * @returns {Jwk | undefined}
    */
    jwk() {
        const ret = wasm.jwsheader_jwk(this.__wbg_ptr);
        return ret === 0 ? undefined : Jwk.__wrap(ret);
    }
    /**
    * Sets a value for the JWK claim (jwk).
    * @param {Jwk} value
    */
    setJwk(value) {
        _assertClass(value, Jwk);
        wasm.jwsheader_setJwk(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
    * Returns the value of the key ID claim (kid).
    * @returns {string | undefined}
    */
    kid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the key ID claim (kid).
    * @param {string} value
    */
    setKid(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setKid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Returns the value of the X.509 URL claim (x5u).
    * @returns {string | undefined}
    */
    x5u() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_x5u(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the X.509 URL claim (x5u).
    * @param {string} value
    */
    setX5u(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.jwsheader_setX5u(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the X.509 certificate chain claim (x5c).
    * @returns {Array<string>}
    */
    x5c() {
        const ret = wasm.jwsheader_crit(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Sets values for the X.509 certificate chain claim (x5c).
    * @param {Array<string>} value
    */
    setX5c(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_setX5c(retptr, this.__wbg_ptr, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the X.509 certificate SHA-1 thumbprint claim (x5t).
    * @returns {string | undefined}
    */
    x5t() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_x5t(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the X.509 certificate SHA-1 thumbprint claim (x5t).
    * @param {string} value
    */
    setX5t(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setX5t(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Returns the value of the X.509 certificate SHA-256 thumbprint claim
    * (x5t#S256).
    * @returns {string | undefined}
    */
    x5tS256() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_x5tS256(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the X.509 certificate SHA-256 thumbprint claim
    * (x5t#S256).
    * @param {string} value
    */
    setX5tS256(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setX5tS256(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Returns the value of the token type claim (typ).
    * @returns {string | undefined}
    */
    typ() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_typ(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the token type claim (typ).
    * @param {string} value
    */
    setTyp(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setTyp(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Returns the value of the content type claim (cty).
    * @returns {string | undefined}
    */
    cty() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_cty(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the content type claim (cty).
    * @param {string} value
    */
    setCty(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setCty(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Returns the value of the critical claim (crit).
    * @returns {Array<string>}
    */
    crit() {
        const ret = wasm.jwsheader_crit(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Sets values for the critical claim (crit).
    * @param {Array<string>} value
    */
    setCrit(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_setCrit(retptr, this.__wbg_ptr, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the url claim (url).
    * @returns {string | undefined}
    */
    url() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_url(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the url claim (url).
    * @param {string} value
    */
    setUrl(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.jwsheader_setUrl(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the nonce claim (nonce).
    * @returns {string | undefined}
    */
    nonce() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_nonce(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets a value for the nonce claim (nonce).
    * @param {string} value
    */
    setNonce(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsheader_setNonce(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwsHeader}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsheader_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwsHeader.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwsHeader}
    */
    clone() {
        const ret = wasm.jwsheader_clone(this.__wbg_ptr);
        return JwsHeader.__wrap(ret);
    }
}
module.exports.JwsHeader = JwsHeader;

const JwsSignatureOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwssignatureoptions_free(ptr >>> 0, 1));
/**
*/
class JwsSignatureOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwsSignatureOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwsSignatureOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwsSignatureOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwssignatureoptions_free(ptr, 0);
    }
    /**
    * @param {IJwsSignatureOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwssignatureoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JwsSignatureOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Replace the value of the `attachJwk` field.
    * @param {boolean} value
    */
    setAttachJwk(value) {
        wasm.jwssignatureoptions_setAttachJwk(this.__wbg_ptr, value);
    }
    /**
    * Replace the value of the `b64` field.
    * @param {boolean} value
    */
    setB64(value) {
        wasm.jwssignatureoptions_setB64(this.__wbg_ptr, value);
    }
    /**
    * Replace the value of the `typ` field.
    * @param {string} value
    */
    setTyp(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwssignatureoptions_setTyp(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Replace the value of the `cty` field.
    * @param {string} value
    */
    setCty(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwssignatureoptions_setCty(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Replace the value of the `url` field.
    * @param {string} value
    */
    serUrl(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.jwssignatureoptions_serUrl(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Replace the value of the `nonce` field.
    * @param {string} value
    */
    setNonce(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwssignatureoptions_setNonce(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Replace the value of the `kid` field.
    * @param {string} value
    */
    setKid(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwssignatureoptions_setKid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Replace the value of the `detached_payload` field.
    * @param {boolean} value
    */
    setDetachedPayload(value) {
        wasm.jwssignatureoptions_setDetachedPayload(this.__wbg_ptr, value);
    }
    /**
    * Add additional header parameters.
    * @param {Record<string, any>} value
    */
    setCustomHeaderParameters(value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwssignatureoptions_setCustomHeaderParameters(retptr, this.__wbg_ptr, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwssignatureoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwsSignatureOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwssignatureoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwsSignatureOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwsSignatureOptions}
    */
    clone() {
        const ret = wasm.jwssignatureoptions_clone(this.__wbg_ptr);
        return JwsSignatureOptions.__wrap(ret);
    }
}
module.exports.JwsSignatureOptions = JwsSignatureOptions;

const JwsVerificationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwsverificationoptions_free(ptr >>> 0, 1));
/**
*/
class JwsVerificationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwsVerificationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwsVerificationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwsVerificationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwsverificationoptions_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwsVerificationOptions} from the given fields.
    * @param {IJwsVerificationOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsverificationoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JwsVerificationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the expected value for the `nonce` parameter of the protected header.
    * @param {string} value
    */
    setNonce(value) {
        const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.jwsverificationoptions_setNonce(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Set the scope of the verification methods that may be used to verify the given JWS.
    * @param {MethodScope} value
    */
    setMethodScope(value) {
        _assertClass(value, MethodScope);
        wasm.jwsverificationoptions_setMethodScope(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
    * Set the DID URl of the method, whose JWK should be used to verify the JWS.
    * @param {DIDUrl} value
    */
    setMethodId(value) {
        _assertClass(value, DIDUrl);
        wasm.jwsverificationoptions_setMethodId(this.__wbg_ptr, value.__wbg_ptr);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsverificationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwsVerificationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwsverificationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwsVerificationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwsVerificationOptions}
    */
    clone() {
        const ret = wasm.jwsverificationoptions_clone(this.__wbg_ptr);
        return JwsVerificationOptions.__wrap(ret);
    }
}
module.exports.JwsVerificationOptions = JwsVerificationOptions;

const JwtFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwt_free(ptr >>> 0, 1));
/**
* A wrapper around a JSON Web Token (JWK).
*/
class Jwt {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Jwt.prototype);
        obj.__wbg_ptr = ptr;
        JwtFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwt_free(ptr, 0);
    }
    /**
    * Creates a new {@link Jwt} from the given string.
    * @param {string} jwt_string
    */
    constructor(jwt_string) {
        const ptr0 = passStringToWasm0(jwt_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.jwt_constructor(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        JwtFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Returns a clone of the JWT string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwt_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwt_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Jwt}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwt_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Jwt.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Jwt}
    */
    clone() {
        const ret = wasm.jwt_clone(this.__wbg_ptr);
        return Jwt.__wrap(ret);
    }
}
module.exports.Jwt = Jwt;

const JwtCredentialValidationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtcredentialvalidationoptions_free(ptr >>> 0, 1));
/**
* Options to declare validation criteria when validating credentials.
*/
class JwtCredentialValidationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwtCredentialValidationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwtCredentialValidationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtCredentialValidationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtcredentialvalidationoptions_free(ptr, 0);
    }
    /**
    * @param {IJwtCredentialValidationOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtcredentialvalidationoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JwtCredentialValidationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtcredentialvalidationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwtCredentialValidationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtcredentialvalidationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwtCredentialValidationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwtCredentialValidationOptions}
    */
    clone() {
        const ret = wasm.jwtcredentialvalidationoptions_clone(this.__wbg_ptr);
        return JwtCredentialValidationOptions.__wrap(ret);
    }
}
module.exports.JwtCredentialValidationOptions = JwtCredentialValidationOptions;

const JwtCredentialValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtcredentialvalidator_free(ptr >>> 0, 1));
/**
* A type for decoding and validating {@link Credential}.
*/
class JwtCredentialValidator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtCredentialValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtcredentialvalidator_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwtCredentialValidator}. If a `signatureVerifier` is provided it will be used when
    * verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
    * algorithm will be used.
    * @param {IJwsVerifier} signatureVerifier
    */
    constructor(signatureVerifier) {
        const ret = wasm.jwtcredentialvalidator_new(addHeapObject(signatureVerifier));
        this.__wbg_ptr = ret >>> 0;
        JwtCredentialValidatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Decodes and validates a {@link Credential} issued as a JWS. A {@link DecodedJwtCredential} is returned upon
    * success.
    *
    * The following properties are validated according to `options`:
    * - the issuer's signature on the JWS,
    * - the expiration date,
    * - the issuance date,
    * - the semantic structure.
    *
    * # Warning
    * The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
    * trusted. This section contains more information on additional checks that should be carried out before and after
    * calling this method.
    *
    * ## The state of the issuer's DID Document
    * The caller must ensure that `issuer` represents an up-to-date DID Document.
    *
    * ## Properties that are not validated
    *  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
    * `proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
    * These should be manually checked after validation, according to your requirements.
    *
    * # Errors
    * An error is returned whenever a validated condition is not satisfied.
    * @param {Jwt} credential_jwt
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {JwtCredentialValidationOptions} options
    * @param {FailFast} fail_fast
    * @returns {DecodedJwtCredential}
    */
    validate(credential_jwt, issuer, options, fail_fast) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential_jwt, Jwt);
            _assertClass(options, JwtCredentialValidationOptions);
            wasm.jwtcredentialvalidator_validate(retptr, this.__wbg_ptr, credential_jwt.__wbg_ptr, addBorrowedObject(issuer), options.__wbg_ptr, fail_fast);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJwtCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Decode and verify the JWS signature of a {@link Credential} issued as a JWT using the DID Document of a trusted
    * issuer.
    *
    * A {@link DecodedJwtCredential} is returned upon success.
    *
    * # Warning
    * The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
    *
    * ## Proofs
    *  Only the JWS signature is verified. If the {@link Credential} contains a `proof` property this will not be
    * verified by this method.
    *
    * # Errors
    * This method immediately returns an error if
    * the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
    * to verify the credential's signature will be made and an error is returned upon failure.
    * @param {Jwt} credential
    * @param {Array<CoreDocument | IToCoreDocument>} trustedIssuers
    * @param {JwsVerificationOptions} options
    * @returns {DecodedJwtCredential}
    */
    verifySignature(credential, trustedIssuers, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Jwt);
            _assertClass(options, JwsVerificationOptions);
            wasm.jwtcredentialvalidator_verifySignature(retptr, this.__wbg_ptr, credential.__wbg_ptr, addBorrowedObject(trustedIssuers), options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJwtCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Validate that the credential expires on or after the specified timestamp.
    * @param {Credential} credential
    * @param {Timestamp} timestamp
    */
    static checkExpiresOnOrAfter(credential, timestamp) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            _assertClass(timestamp, Timestamp);
            wasm.jwtcredentialvalidator_checkExpiresOnOrAfter(retptr, credential.__wbg_ptr, timestamp.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Validate that the credential is issued on or before the specified timestamp.
    * @param {Credential} credential
    * @param {Timestamp} timestamp
    */
    static checkIssuedOnOrBefore(credential, timestamp) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            _assertClass(timestamp, Timestamp);
            wasm.jwtcredentialvalidator_checkIssuedOnOrBefore(retptr, credential.__wbg_ptr, timestamp.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Validate that the relationship between the `holder` and the credential subjects is in accordance with
    * `relationship`. The `holder` parameter is expected to be the URL of the holder.
    * @param {Credential} credential
    * @param {string} holder
    * @param {SubjectHolderRelationship} relationship
    */
    static checkSubjectHolderRelationship(credential, holder, relationship) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            const ptr0 = passStringToWasm0(holder, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.jwtcredentialvalidator_checkSubjectHolderRelationship(retptr, credential.__wbg_ptr, ptr0, len0, relationship);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Checks whether the credential status has been revoked.
    *
    * Only supports `RevocationBitmap2022`.
    * @param {Credential} credential
    * @param {Array<CoreDocument | IToCoreDocument>} trustedIssuers
    * @param {StatusCheck} statusCheck
    */
    static checkStatus(credential, trustedIssuers, statusCheck) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            wasm.jwtcredentialvalidator_checkStatus(retptr, credential.__wbg_ptr, addBorrowedObject(trustedIssuers), statusCheck);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Checks wheter the credential status has been revoked using `StatusList2021`.
    * @param {Credential} credential
    * @param {StatusList2021Credential} status_list
    * @param {StatusCheck} status_check
    */
    static checkStatusWithStatusList2021(credential, status_list, status_check) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            _assertClass(status_list, StatusList2021Credential);
            wasm.jwtcredentialvalidator_checkStatusWithStatusList2021(retptr, credential.__wbg_ptr, status_list.__wbg_ptr, status_check);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Utility for extracting the issuer field of a {@link Credential} as a DID.
    *
    * ### Errors
    *
    * Fails if the issuer field is not a valid DID.
    * @param {Credential} credential
    * @returns {CoreDID}
    */
    static extractIssuer(credential) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            wasm.jwtcredentialvalidator_extractIssuer(retptr, credential.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Utility for extracting the issuer field of a credential in JWT representation as DID.
    *
    * # Errors
    *
    * If the JWT decoding fails or the issuer field is not a valid DID.
    * @param {Jwt} credential
    * @returns {CoreDID}
    */
    static extractIssuerFromJwt(credential) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Jwt);
            wasm.jwtcredentialvalidator_extractIssuerFromJwt(retptr, credential.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JwtCredentialValidator = JwtCredentialValidator;

const JwtDomainLinkageValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtdomainlinkagevalidator_free(ptr >>> 0, 1));
/**
* A validator for a Domain Linkage Configuration and Credentials.
*/
class JwtDomainLinkageValidator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtDomainLinkageValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtdomainlinkagevalidator_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwtDomainLinkageValidator}. If a `signatureVerifier` is provided it will be used when
    * verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
    * algorithm will be used.
    * @param {IJwsVerifier} signatureVerifier
    */
    constructor(signatureVerifier) {
        const ret = wasm.jwtdomainlinkagevalidator_new(addHeapObject(signatureVerifier));
        this.__wbg_ptr = ret >>> 0;
        JwtDomainLinkageValidatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Validates the linkage between a domain and a DID.
    * {@link DomainLinkageConfiguration} is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).
    *
    * Linkage is valid if no error is thrown.
    *
    * # Note:
    * - Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)
    *   is supported.
    * - Only the Credential issued by `issuer` is verified.
    *
    * # Errors
    *
    *  - Semantic structure of `configuration` is invalid.
    *  - `configuration` includes multiple credentials issued by `issuer`.
    *  - Validation of the matched Domain Linkage Credential fails.
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {DomainLinkageConfiguration} configuration
    * @param {string} domain
    * @param {JwtCredentialValidationOptions} options
    */
    validateLinkage(issuer, configuration, domain, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(configuration, DomainLinkageConfiguration);
            const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(options, JwtCredentialValidationOptions);
            wasm.jwtdomainlinkagevalidator_validateLinkage(retptr, this.__wbg_ptr, addBorrowedObject(issuer), configuration.__wbg_ptr, ptr0, len0, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
    *
    * Error will be thrown in case the validation fails.
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {Jwt} credentialJwt
    * @param {string} domain
    * @param {JwtCredentialValidationOptions} options
    */
    validateCredential(issuer, credentialJwt, domain, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credentialJwt, Jwt);
            const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(options, JwtCredentialValidationOptions);
            wasm.jwtdomainlinkagevalidator_validateCredential(retptr, this.__wbg_ptr, addBorrowedObject(issuer), credentialJwt.__wbg_ptr, ptr0, len0, options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.JwtDomainLinkageValidator = JwtDomainLinkageValidator;

const JwtPresentationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtpresentationoptions_free(ptr >>> 0, 1));
/**
*/
class JwtPresentationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwtPresentationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwtPresentationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtPresentationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtpresentationoptions_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwtPresentationOptions} from the given fields.
    *
    * Throws an error if any of the options are invalid.
    * @param {IJwtPresentationOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JwtPresentationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwtPresentationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwtPresentationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwtPresentationOptions}
    */
    clone() {
        const ret = wasm.jwtpresentationoptions_clone(this.__wbg_ptr);
        return JwtPresentationOptions.__wrap(ret);
    }
}
module.exports.JwtPresentationOptions = JwtPresentationOptions;

const JwtPresentationValidationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtpresentationvalidationoptions_free(ptr >>> 0, 1));
/**
* Options to declare validation criteria when validating presentation.
*/
class JwtPresentationValidationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JwtPresentationValidationOptions.prototype);
        obj.__wbg_ptr = ptr;
        JwtPresentationValidationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtPresentationValidationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtpresentationvalidationoptions_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwtPresentationValidationOptions} from the given fields.
    *
    * Throws an error if any of the options are invalid.
    * @param {IJwtPresentationValidationOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationvalidationoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            JwtPresentationValidationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationvalidationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {JwtPresentationValidationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.jwtpresentationvalidationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return JwtPresentationValidationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {JwtPresentationValidationOptions}
    */
    clone() {
        const ret = wasm.jwtpresentationvalidationoptions_clone(this.__wbg_ptr);
        return JwtPresentationValidationOptions.__wrap(ret);
    }
}
module.exports.JwtPresentationValidationOptions = JwtPresentationValidationOptions;

const JwtPresentationValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_jwtpresentationvalidator_free(ptr >>> 0, 1));
/**
*/
class JwtPresentationValidator {

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JwtPresentationValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jwtpresentationvalidator_free(ptr, 0);
    }
    /**
    * Creates a new {@link JwtPresentationValidator}. If a `signatureVerifier` is provided it will be used when
    * verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
    * algorithm will be used.
    * @param {IJwsVerifier} signatureVerifier
    */
    constructor(signatureVerifier) {
        const ret = wasm.jwtpresentationvalidator_new(addHeapObject(signatureVerifier));
        this.__wbg_ptr = ret >>> 0;
        JwtPresentationValidatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Validates a {@link Presentation} encoded as a {@link Jwt}.
    *
    * The following properties are validated according to `options`:
    * - the JWT can be decoded into a semantically valid presentation.
    * - the expiration and issuance date contained in the JWT claims.
    * - the holder's signature.
    *
    * Validation is done with respect to the properties set in `options`.
    *
    * # Warning
    *
    * * This method does NOT validate the constituent credentials and therefore also not the relationship between the
    * credentials' subjects and the presentation holder. This can be done with {@link JwtCredentialValidationOptions}.
    * * The lack of an error returned from this method is in of itself not enough to conclude that the presentation can
    * be trusted. This section contains more information on additional checks that should be carried out before and
    * after calling this method.
    *
    * ## The state of the supplied DID Documents.
    *
    * The caller must ensure that the DID Documents in `holder` are up-to-date.
    *
    * # Errors
    *
    * An error is returned whenever a validated condition is not satisfied or when decoding fails.
    * @param {Jwt} presentationJwt
    * @param {CoreDocument | IToCoreDocument} holder
    * @param {JwtPresentationValidationOptions} validation_options
    * @returns {DecodedJwtPresentation}
    */
    validate(presentationJwt, holder, validation_options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentationJwt, Jwt);
            _assertClass(validation_options, JwtPresentationValidationOptions);
            wasm.jwtpresentationvalidator_validate(retptr, this.__wbg_ptr, presentationJwt.__wbg_ptr, addBorrowedObject(holder), validation_options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJwtPresentation.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Validates the semantic structure of the {@link Presentation}.
    * @param {Presentation} presentation
    */
    static checkStructure(presentation) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation, Presentation);
            wasm.jwtpresentationvalidator_checkStructure(retptr, presentation.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Attempt to extract the holder of the presentation.
    *
    * # Errors:
    * * If deserialization/decoding of the presentation fails.
    * * If the holder can't be parsed as DIDs.
    * @param {Jwt} presentation
    * @returns {CoreDID}
    */
    static extractHolder(presentation) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(presentation, Jwt);
            wasm.jwtpresentationvalidator_extractHolder(retptr, presentation.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CoreDID.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.JwtPresentationValidator = JwtPresentationValidator;

const KeyBindingJWTValidationOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_keybindingjwtvalidationoptions_free(ptr >>> 0, 1));
/**
* Options to declare validation criteria when validating credentials.
*/
class KeyBindingJWTValidationOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(KeyBindingJWTValidationOptions.prototype);
        obj.__wbg_ptr = ptr;
        KeyBindingJWTValidationOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeyBindingJWTValidationOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_keybindingjwtvalidationoptions_free(ptr, 0);
    }
    /**
    * @param {IKeyBindingJWTValidationOptions | undefined} [options]
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtvalidationoptions_new(retptr, isLikeNone(options) ? 0 : addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            KeyBindingJWTValidationOptionsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtvalidationoptions_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {KeyBindingJWTValidationOptions}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtvalidationoptions_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return KeyBindingJWTValidationOptions.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {KeyBindingJWTValidationOptions}
    */
    clone() {
        const ret = wasm.keybindingjwtvalidationoptions_clone(this.__wbg_ptr);
        return KeyBindingJWTValidationOptions.__wrap(ret);
    }
}
module.exports.KeyBindingJWTValidationOptions = KeyBindingJWTValidationOptions;

const KeyBindingJwtClaimsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_keybindingjwtclaims_free(ptr >>> 0, 1));
/**
* Claims set for key binding JWT.
*/
class KeyBindingJwtClaims {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(KeyBindingJwtClaims.prototype);
        obj.__wbg_ptr = ptr;
        KeyBindingJwtClaimsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        KeyBindingJwtClaimsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_keybindingjwtclaims_free(ptr, 0);
    }
    /**
    * Creates a new [`KeyBindingJwtClaims`].
    * When `issued_at` is left as None, it will automatically default to the current time.
    *
    * # Error
    * When `issued_at` is set to `None` and the system returns time earlier than `SystemTime::UNIX_EPOCH`.
    * @param {string} jwt
    * @param {Array<string>} disclosures
    * @param {string} nonce
    * @param {string} aud
    * @param {Timestamp | undefined} [issued_at]
    * @param {Record<string, any> | undefined} [custom_properties]
    */
    constructor(jwt, disclosures, nonce, aud, issued_at, custom_properties) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(jwt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(nonce, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(aud, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len2 = WASM_VECTOR_LEN;
            let ptr3 = 0;
            if (!isLikeNone(issued_at)) {
                _assertClass(issued_at, Timestamp);
                ptr3 = issued_at.__destroy_into_raw();
            }
            wasm.keybindingjwtclaims_new(retptr, ptr0, len0, addHeapObject(disclosures), ptr1, len1, ptr2, len2, ptr3, isLikeNone(custom_properties) ? 0 : addHeapObject(custom_properties));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            KeyBindingJwtClaimsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a string representation of the claims.
    * @returns {string}
    */
    toString() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Returns a copy of the issued at `iat` property.
    * @returns {bigint}
    */
    iat() {
        const ret = wasm.keybindingjwtclaims_iat(this.__wbg_ptr);
        return ret;
    }
    /**
    * Returns a copy of the audience `aud` property.
    * @returns {string}
    */
    aud() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_aud(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the `nonce` property.
    * @returns {string}
    */
    nonce() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_nonce(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the `sd_hash` property.
    * @returns {string}
    */
    sdHash() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_sdHash(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the custom properties.
    * @returns {Record<string, any>}
    */
    customProperties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_customProperties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the value of the `typ` property of the JWT header according to
    * https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-key-binding-jwt
    * @returns {string}
    */
    static keyBindingJwtHeaderTyp() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_keyBindingJwtHeaderTyp(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {KeyBindingJwtClaims}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.keybindingjwtclaims_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return KeyBindingJwtClaims.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {KeyBindingJwtClaims}
    */
    clone() {
        const ret = wasm.keybindingjwtclaims_clone(this.__wbg_ptr);
        return KeyBindingJwtClaims.__wrap(ret);
    }
}
module.exports.KeyBindingJwtClaims = KeyBindingJwtClaims;

const LinkedDomainServiceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_linkeddomainservice_free(ptr >>> 0, 1));
/**
*/
class LinkedDomainService {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LinkedDomainService.prototype);
        obj.__wbg_ptr = ptr;
        LinkedDomainServiceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LinkedDomainServiceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_linkeddomainservice_free(ptr, 0);
    }
    /**
    * Constructs a new {@link LinkedDomainService} that wraps a spec compliant [Linked Domain Service Endpoint](https://identity.foundation/.well-known/resources/did-configuration/#linked-domain-service-endpoint).
    *
    * Domain URLs must include the `https` scheme in order to pass the domain linkage validation.
    * @param {ILinkedDomainService} options
    */
    constructor(options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.linkeddomainservice_new(retptr, addHeapObject(options));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            LinkedDomainServiceFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the domains contained in the Linked Domain Service.
    * @returns {Array<string>}
    */
    domains() {
        const ret = wasm.linkeddomainservice_domains(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the inner service which can be added to a DID Document.
    * @returns {Service}
    */
    toService() {
        const ret = wasm.linkeddomainservice_toService(this.__wbg_ptr);
        return Service.__wrap(ret);
    }
    /**
    * Creates a new {@link LinkedDomainService} from a {@link Service}.
    *
    * # Error
    *
    * Errors if `service` is not a valid Linked Domain Service.
    * @param {Service} service
    * @returns {LinkedDomainService}
    */
    static fromService(service) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(service, Service);
            wasm.linkeddomainservice_fromService(retptr, service.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return LinkedDomainService.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns `true` if a {@link Service} is a valid Linked Domain Service.
    * @param {Service} service
    * @returns {boolean}
    */
    static isValid(service) {
        _assertClass(service, Service);
        const ret = wasm.linkeddomainservice_isValid(service.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * Deep clones the object.
    * @returns {LinkedDomainService}
    */
    clone() {
        const ret = wasm.linkeddomainservice_clone(this.__wbg_ptr);
        return LinkedDomainService.__wrap(ret);
    }
}
module.exports.LinkedDomainService = LinkedDomainService;

const MethodDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_methoddata_free(ptr >>> 0, 1));
/**
* Supported verification method data formats.
*/
class MethodData {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MethodData.prototype);
        obj.__wbg_ptr = ptr;
        MethodDataFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MethodDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_methoddata_free(ptr, 0);
    }
    /**
    * Creates a new {@link MethodData} variant with Base58-BTC encoded content.
    * @param {Uint8Array} data
    * @returns {MethodData}
    */
    static newBase58(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.methoddata_newBase58(ptr0, len0);
        return MethodData.__wrap(ret);
    }
    /**
    * Creates a new {@link MethodData} variant with Multibase-encoded content.
    * @param {Uint8Array} data
    * @returns {MethodData}
    */
    static newMultibase(data) {
        const ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.methoddata_newMultibase(ptr0, len0);
        return MethodData.__wrap(ret);
    }
    /**
    * Creates a new {@link MethodData} variant consisting of the given `key`.
    *
    * ### Errors
    * An error is thrown if the given `key` contains any private components.
    * @param {Jwk} key
    * @returns {MethodData}
    */
    static newJwk(key) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(key, Jwk);
            wasm.methoddata_newJwk(retptr, key.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Creates a new custom {@link MethodData}.
    * @param {string} name
    * @param {any} data
    * @returns {MethodData}
    */
    static newCustom(name, data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.methoddata_newCustom(retptr, ptr0, len0, addHeapObject(data));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the wrapped custom method data format is `Custom`.
    * @returns {CustomMethodData}
    */
    tryCustom() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddata_tryCustom(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return CustomMethodData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a `Uint8Array` containing the decoded bytes of the {@link MethodData}.
    *
    * This is generally a public key identified by a {@link MethodData} value.
    *
    * ### Errors
    * Decoding can fail if {@link MethodData} has invalid content or cannot be
    * represented as a vector of bytes.
    * @returns {Uint8Array}
    */
    tryDecode() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddata_tryDecode(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the wrapped {@link Jwk} if the format is `PublicKeyJwk`.
    * @returns {Jwk}
    */
    tryPublicKeyJwk() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddata_tryPublicKeyJwk(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Jwk.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddata_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {MethodData}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddata_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodData.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {MethodData}
    */
    clone() {
        const ret = wasm.methoddata_clone(this.__wbg_ptr);
        return MethodData.__wrap(ret);
    }
}
module.exports.MethodData = MethodData;

const MethodDigestFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_methoddigest_free(ptr >>> 0, 1));
/**
* Unique identifier of a {@link VerificationMethod}.
*
* NOTE:
* This class does not have a JSON representation,
* use the methods `pack` and `unpack` instead.
*/
class MethodDigest {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MethodDigest.prototype);
        obj.__wbg_ptr = ptr;
        MethodDigestFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MethodDigestFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_methoddigest_free(ptr, 0);
    }
    /**
    * @param {VerificationMethod} verification_method
    */
    constructor(verification_method) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(verification_method, VerificationMethod);
            wasm.methoddigest_new(retptr, verification_method.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            MethodDigestFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Packs {@link MethodDigest} into bytes.
    * @returns {Uint8Array}
    */
    pack() {
        const ret = wasm.methoddigest_pack(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Unpacks bytes into {@link MethodDigest}.
    * @param {Uint8Array} bytes
    * @returns {MethodDigest}
    */
    static unpack(bytes) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methoddigest_unpack(retptr, addBorrowedObject(bytes));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodDigest.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {MethodDigest}
    */
    clone() {
        const ret = wasm.methoddigest_clone(this.__wbg_ptr);
        return MethodDigest.__wrap(ret);
    }
}
module.exports.MethodDigest = MethodDigest;

const MethodScopeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_methodscope_free(ptr >>> 0, 1));
/**
* Supported verification method types.
*/
class MethodScope {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MethodScope.prototype);
        obj.__wbg_ptr = ptr;
        MethodScopeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MethodScopeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_methodscope_free(ptr, 0);
    }
    /**
    * @returns {MethodScope}
    */
    static VerificationMethod() {
        const ret = wasm.methodscope_VerificationMethod();
        return MethodScope.__wrap(ret);
    }
    /**
    * @returns {MethodScope}
    */
    static Authentication() {
        const ret = wasm.methodscope_Authentication();
        return MethodScope.__wrap(ret);
    }
    /**
    * @returns {MethodScope}
    */
    static AssertionMethod() {
        const ret = wasm.methodscope_AssertionMethod();
        return MethodScope.__wrap(ret);
    }
    /**
    * @returns {MethodScope}
    */
    static KeyAgreement() {
        const ret = wasm.methodscope_KeyAgreement();
        return MethodScope.__wrap(ret);
    }
    /**
    * @returns {MethodScope}
    */
    static CapabilityDelegation() {
        const ret = wasm.methodscope_CapabilityDelegation();
        return MethodScope.__wrap(ret);
    }
    /**
    * @returns {MethodScope}
    */
    static CapabilityInvocation() {
        const ret = wasm.methodscope_CapabilityInvocation();
        return MethodScope.__wrap(ret);
    }
    /**
    * Returns the {@link MethodScope} as a string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodscope_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodscope_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {MethodScope}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodscope_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodScope.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {MethodScope}
    */
    clone() {
        const ret = wasm.methodscope_clone(this.__wbg_ptr);
        return MethodScope.__wrap(ret);
    }
}
module.exports.MethodScope = MethodScope;

const MethodTypeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_methodtype_free(ptr >>> 0, 1));
/**
* Supported verification method types.
*/
class MethodType {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MethodType.prototype);
        obj.__wbg_ptr = ptr;
        MethodTypeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MethodTypeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_methodtype_free(ptr, 0);
    }
    /**
    * @returns {MethodType}
    */
    static Ed25519VerificationKey2018() {
        const ret = wasm.methodtype_Ed25519VerificationKey2018();
        return MethodType.__wrap(ret);
    }
    /**
    * @returns {MethodType}
    */
    static X25519KeyAgreementKey2019() {
        const ret = wasm.methodtype_X25519KeyAgreementKey2019();
        return MethodType.__wrap(ret);
    }
    /**
    * @deprecated Use {@link JsonWebKey2020} instead.
    */
    static JsonWebKey() {
        const ret = wasm.methodtype_JsonWebKey();
        return MethodType.__wrap(ret);
    }
    /**
    * A verification method for use with JWT verification as prescribed by the {@link Jwk}
    * in the `publicKeyJwk` entry.
    * @returns {MethodType}
    */
    static JsonWebKey2020() {
        const ret = wasm.methodtype_JsonWebKey2020();
        return MethodType.__wrap(ret);
    }
    /**
    * A custom method.
    * @param {string} type_
    * @returns {MethodType}
    */
    static custom(type_) {
        const ptr0 = passStringToWasm0(type_, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.methodtype_custom(ptr0, len0);
        return MethodType.__wrap(ret);
    }
    /**
    * Returns the {@link MethodType} as a string.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodtype_toString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodtype_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {MethodType}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.methodtype_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return MethodType.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {MethodType}
    */
    clone() {
        const ret = wasm.methodtype_clone(this.__wbg_ptr);
        return MethodType.__wrap(ret);
    }
}
module.exports.MethodType = MethodType;

const PayloadEntryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_payloadentry_free(ptr >>> 0, 1));
/**
*/
class PayloadEntry {

    static __unwrap(jsValue) {
        if (!(jsValue instanceof PayloadEntry)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PayloadEntryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_payloadentry_free(ptr, 0);
    }
    /**
    * @returns {PayloadType}
    */
    get 1() {
        const ret = wasm.__wbg_get_payloadentry_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {PayloadType} arg0
    */
    set 1(arg0) {
        wasm.__wbg_set_payloadentry_1(this.__wbg_ptr, arg0);
    }
    /**
    * @param {any} value
    */
    set value(value) {
        wasm.payloadentry_set_value(this.__wbg_ptr, addHeapObject(value));
    }
    /**
    * @returns {any}
    */
    get value() {
        const ret = wasm.payloadentry_value(this.__wbg_ptr);
        return takeObject(ret);
    }
}
module.exports.PayloadEntry = PayloadEntry;

const PayloadsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_payloads_free(ptr >>> 0, 1));
/**
*/
class Payloads {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Payloads.prototype);
        obj.__wbg_ptr = ptr;
        PayloadsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PayloadsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_payloads_free(ptr, 0);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Payloads}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Payloads.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Payloads}
    */
    clone() {
        const ret = wasm.payloads_clone(this.__wbg_ptr);
        return Payloads.__wrap(ret);
    }
    /**
    * @param {(PayloadEntry)[]} entries
    */
    constructor(entries) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayJsValueToWasm0(entries, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.payloads_new(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            PayloadsFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {any[]} values
    * @returns {Payloads}
    */
    static newFromValues(values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayJsValueToWasm0(values, wasm.__wbindgen_malloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.payloads_newFromValues(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Payloads.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any[]}
    */
    getValues() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_getValues(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint32Array}
    */
    getUndisclosedIndexes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_getUndisclosedIndexes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Uint32Array}
    */
    getDisclosedIndexes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_getDisclosedIndexes(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any[]}
    */
    getUndisclosedPayloads() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_getUndisclosedPayloads(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Payloads}
    */
    getDisclosedPayloads() {
        const ret = wasm.payloads_getDisclosedPayloads(this.__wbg_ptr);
        return Payloads.__wrap(ret);
    }
    /**
    * @param {number} index
    */
    setUndisclosed(index) {
        wasm.payloads_setUndisclosed(this.__wbg_ptr, index);
    }
    /**
    * @param {number} index
    * @param {any} value
    * @returns {any}
    */
    replacePayloadAtIndex(index, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.payloads_replacePayloadAtIndex(retptr, this.__wbg_ptr, index, addHeapObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.Payloads = Payloads;

const PresentationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_presentation_free(ptr >>> 0, 1));
/**
*/
class Presentation {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Presentation.prototype);
        obj.__wbg_ptr = ptr;
        PresentationFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PresentationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_presentation_free(ptr, 0);
    }
    /**
    * Returns the base JSON-LD context.
    * @returns {string}
    */
    static BaseContext() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_BaseContext(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Returns the base type.
    * @returns {string}
    */
    static BaseType() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_BaseType(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Constructs a new presentation.
    * @param {IPresentation} values
    */
    constructor(values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_new(retptr, addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            PresentationFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the JSON-LD context(s) applicable to the presentation.
    * @returns {Array<string | Record<string, any>>}
    */
    context() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_context(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the unique `URI` identifying the presentation.
    * @returns {string | undefined}
    */
    id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the URIs defining the type of the presentation.
    * @returns {Array<string>}
    */
    type() {
        const ret = wasm.presentation_type(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns the JWT credentials expressing the claims of the presentation.
    * @returns {Array<UnknownCredential>}
    */
    verifiableCredential() {
        const ret = wasm.presentation_verifiableCredential(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns a copy of the URI of the entity that generated the presentation.
    * @returns {string}
    */
    holder() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_holder(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns a copy of the service(s) used to refresh an expired {@link Credential} in the presentation.
    * @returns {Array<RefreshService>}
    */
    refreshService() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_refreshService(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the terms-of-use specified by the presentation holder
    * @returns {Array<Policy>}
    */
    termsOfUse() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_termsOfUse(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Optional cryptographic proof, unrelated to JWT.
    * @returns {Proof | undefined}
    */
    proof() {
        const ret = wasm.presentation_proof(this.__wbg_ptr);
        return ret === 0 ? undefined : Proof.__wrap(ret);
    }
    /**
    * Sets the proof property of the {@link Presentation}.
    *
    * Note that this proof is not related to JWT.
    * @param {Proof | undefined} [proof]
    */
    setProof(proof) {
        let ptr0 = 0;
        if (!isLikeNone(proof)) {
            _assertClass(proof, Proof);
            ptr0 = proof.__destroy_into_raw();
        }
        wasm.presentation_setProof(this.__wbg_ptr, ptr0);
    }
    /**
    * Returns a copy of the miscellaneous properties on the presentation.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Presentation}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.presentation_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Presentation.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Presentation}
    */
    clone() {
        const ret = wasm.presentation_clone(this.__wbg_ptr);
        return Presentation.__wrap(ret);
    }
}
module.exports.Presentation = Presentation;

const PresentationProtectedHeaderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_presentationprotectedheader_free(ptr >>> 0, 1));
/**
*/
class PresentationProtectedHeader {

    toJSON() {
        return {
            alg: this.alg,
            kid: this.kid,
            aud: this.aud,
            nonce: this.nonce,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PresentationProtectedHeaderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_presentationprotectedheader_free(ptr, 0);
    }
    /**
    * @returns {PresentationProofAlgorithm}
    */
    get alg() {
        const ret = wasm.__wbg_get_presentationprotectedheader_alg(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {PresentationProofAlgorithm} arg0
    */
    set alg(arg0) {
        wasm.__wbg_set_presentationprotectedheader_alg(this.__wbg_ptr, arg0);
    }
    /**
    * ID for the key used for the JWP.
    * @returns {string | undefined}
    */
    get kid() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_presentationprotectedheader_kid(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * ID for the key used for the JWP.
    * @param {string | undefined} [arg0]
    */
    set kid(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_presentationprotectedheader_kid(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Who have received the JPT.
    * @returns {string | undefined}
    */
    get aud() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_presentationprotectedheader_aud(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Who have received the JPT.
    * @param {string | undefined} [arg0]
    */
    set aud(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_presentationprotectedheader_aud(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * For replay attacks.
    * @returns {string | undefined}
    */
    get nonce() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_presentationprotectedheader_nonce(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * For replay attacks.
    * @param {string | undefined} [arg0]
    */
    set nonce(arg0) {
        var ptr0 = isLikeNone(arg0) ? 0 : passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_presentationprotectedheader_nonce(this.__wbg_ptr, ptr0, len0);
    }
}
module.exports.PresentationProtectedHeader = PresentationProtectedHeader;

const ProofFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_proof_free(ptr >>> 0, 1));
/**
* Represents a cryptographic proof that can be used to validate verifiable credentials and
* presentations.
*
* This representation does not inherently implement any standard; instead, it
* can be utilized to implement standards or user-defined proofs. The presence of the
* `type` field is necessary to accommodate different types of cryptographic proofs.
*
* Note that this proof is not related to JWT and can be used in combination or as an alternative
* to it.
*/
class Proof {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Proof.prototype);
        obj.__wbg_ptr = ptr;
        ProofFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ProofFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_proof_free(ptr, 0);
    }
    /**
    * @param {string} type_
    * @param {any} properties
    */
    constructor(type_, properties) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(type_, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.proof_constructor(retptr, ptr0, len0, addHeapObject(properties));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            ProofFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the type of proof.
    * @returns {string}
    */
    type() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.proof_type(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the properties of the proof.
    * @returns {any}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.proof_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.proof_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Proof}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.proof_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Proof.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Proof}
    */
    clone() {
        const ret = wasm.proof_clone(this.__wbg_ptr);
        return Proof.__wrap(ret);
    }
}
module.exports.Proof = Proof;

const ProofUpdateCtxFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_proofupdatectx_free(ptr >>> 0, 1));
/**
*/
class ProofUpdateCtx {

    toJSON() {
        return {
            old_start_validity_timeframe: this.old_start_validity_timeframe,
            new_start_validity_timeframe: this.new_start_validity_timeframe,
            old_end_validity_timeframe: this.old_end_validity_timeframe,
            new_end_validity_timeframe: this.new_end_validity_timeframe,
            index_start_validity_timeframe: this.index_start_validity_timeframe,
            index_end_validity_timeframe: this.index_end_validity_timeframe,
            number_of_signed_messages: this.number_of_signed_messages,
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ProofUpdateCtxFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_proofupdatectx_free(ptr, 0);
    }
    /**
    * Old `startValidityTimeframe` value
    * @returns {Uint8Array}
    */
    get old_start_validity_timeframe() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_proofupdatectx_old_start_validity_timeframe(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Old `startValidityTimeframe` value
    * @param {Uint8Array} arg0
    */
    set old_start_validity_timeframe(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_proofupdatectx_old_start_validity_timeframe(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * New `startValidityTimeframe` value to be signed
    * @returns {Uint8Array}
    */
    get new_start_validity_timeframe() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_proofupdatectx_new_start_validity_timeframe(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * New `startValidityTimeframe` value to be signed
    * @param {Uint8Array} arg0
    */
    set new_start_validity_timeframe(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_proofupdatectx_new_start_validity_timeframe(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Old `endValidityTimeframe` value
    * @returns {Uint8Array}
    */
    get old_end_validity_timeframe() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_proofupdatectx_old_end_validity_timeframe(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Old `endValidityTimeframe` value
    * @param {Uint8Array} arg0
    */
    set old_end_validity_timeframe(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_proofupdatectx_old_end_validity_timeframe(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * New `endValidityTimeframe` value to be signed
    * @returns {Uint8Array}
    */
    get new_end_validity_timeframe() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_proofupdatectx_new_end_validity_timeframe(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1, 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * New `endValidityTimeframe` value to be signed
    * @param {Uint8Array} arg0
    */
    set new_end_validity_timeframe(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_proofupdatectx_new_end_validity_timeframe(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * Index of `startValidityTimeframe` claim inside the array of Claims
    * @returns {number}
    */
    get index_start_validity_timeframe() {
        const ret = wasm.__wbg_get_proofupdatectx_index_start_validity_timeframe(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Index of `startValidityTimeframe` claim inside the array of Claims
    * @param {number} arg0
    */
    set index_start_validity_timeframe(arg0) {
        wasm.__wbg_set_proofupdatectx_index_start_validity_timeframe(this.__wbg_ptr, arg0);
    }
    /**
    * Index of `endValidityTimeframe` claim inside the array of Claims
    * @returns {number}
    */
    get index_end_validity_timeframe() {
        const ret = wasm.__wbg_get_proofupdatectx_index_end_validity_timeframe(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Index of `endValidityTimeframe` claim inside the array of Claims
    * @param {number} arg0
    */
    set index_end_validity_timeframe(arg0) {
        wasm.__wbg_set_proofupdatectx_index_end_validity_timeframe(this.__wbg_ptr, arg0);
    }
    /**
    * Number of signed messages, number of payloads in a JWP
    * @returns {number}
    */
    get number_of_signed_messages() {
        const ret = wasm.__wbg_get_proofupdatectx_number_of_signed_messages(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Number of signed messages, number of payloads in a JWP
    * @param {number} arg0
    */
    set number_of_signed_messages(arg0) {
        wasm.__wbg_set_proofupdatectx_number_of_signed_messages(this.__wbg_ptr, arg0);
    }
}
module.exports.ProofUpdateCtx = ProofUpdateCtx;

const ResolverFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_resolver_free(ptr >>> 0, 1));
/**
* Convenience type for resolving DID documents from different DID methods.
*
* Also provides methods for resolving DID Documents associated with
* verifiable {@link Credential}s and {@link Presentation}s.
*
* # Configuration
*
* The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.
*/
class Resolver {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ResolverFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_resolver_free(ptr, 0);
    }
    /**
    * Constructs a new {@link Resolver}.
    *
    * # Errors
    * If both a `client` is given and the `handlers` map contains the "iota" key the construction process
    * will throw an error because the handler for the "iota" method then becomes ambiguous.
    * @param {ResolverConfig} config
    */
    constructor(config) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.resolver_new(retptr, addHeapObject(config));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            ResolverFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Fetches the DID Document of the given DID.
    *
    * ### Errors
    *
    * Errors if the resolver has not been configured to handle the method
    * corresponding to the given DID or the resolution process itself fails.
    * @param {string} did
    * @returns {Promise<CoreDocument | IToCoreDocument>}
    */
    resolve(did) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(did, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.resolver_resolve(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Concurrently fetches the DID Documents of the multiple given DIDs.
    *
    * # Errors
    * * If the resolver has not been configured to handle the method of any of the given DIDs.
    * * If the resolution process of any DID fails.
    *
    * ## Note
    * * The order of the documents in the returned array matches that in `dids`.
    * * If `dids` contains duplicates, these will be resolved only once and the resolved document
    * is copied into the returned array to match the order of `dids`.
    * @param {Array<string>} dids
    * @returns {Promise<Array<CoreDocument | IToCoreDocument>>}
    */
    resolveMultiple(dids) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.resolver_resolveMultiple(retptr, this.__wbg_ptr, addHeapObject(dids));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.Resolver = Resolver;

const RevocationBitmapFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_revocationbitmap_free(ptr >>> 0, 1));
/**
* A compressed bitmap for managing credential revocation.
*/
class RevocationBitmap {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RevocationBitmap.prototype);
        obj.__wbg_ptr = ptr;
        RevocationBitmapFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RevocationBitmapFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_revocationbitmap_free(ptr, 0);
    }
    /**
    * Creates a new {@link RevocationBitmap} instance.
    */
    constructor() {
        const ret = wasm.revocationbitmap_new();
        this.__wbg_ptr = ret >>> 0;
        RevocationBitmapFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * The name of the service type.
    * @returns {string}
    */
    static type() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationbitmap_type(retptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns `true` if the credential at the given `index` is revoked.
    * @param {number} index
    * @returns {boolean}
    */
    isRevoked(index) {
        const ret = wasm.revocationbitmap_isRevoked(this.__wbg_ptr, index);
        return ret !== 0;
    }
    /**
    * Mark the given index as revoked.
    *
    * Returns true if the index was absent from the set.
    * @param {number} index
    * @returns {boolean}
    */
    revoke(index) {
        const ret = wasm.revocationbitmap_revoke(this.__wbg_ptr, index);
        return ret !== 0;
    }
    /**
    * Mark the index as not revoked.
    *
    * Returns true if the index was present in the set.
    * @param {number} index
    * @returns {boolean}
    */
    unrevoke(index) {
        const ret = wasm.revocationbitmap_unrevoke(this.__wbg_ptr, index);
        return ret !== 0;
    }
    /**
    * Returns the number of revoked credentials.
    * @returns {number}
    */
    len() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationbitmap_len(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Return a `Service` with:
    * - the service's id set to `serviceId`,
    * - of type `RevocationBitmap2022`,
    * - and with the bitmap embedded in a data url in the service's endpoint.
    * @param {DIDUrl} serviceId
    * @returns {Service}
    */
    toService(serviceId) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(serviceId, DIDUrl);
            wasm.revocationbitmap_toService(retptr, this.__wbg_ptr, serviceId.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Try to construct a {@link RevocationBitmap} from a service
    * if it is a valid Revocation Bitmap Service.
    * @param {Service} service
    * @returns {RevocationBitmap}
    */
    static fromEndpoint(service) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(service, Service);
            wasm.revocationbitmap_fromEndpoint(retptr, service.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return RevocationBitmap.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.RevocationBitmap = RevocationBitmap;

const RevocationTimeframeStatusFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_revocationtimeframestatus_free(ptr >>> 0, 1));
/**
* Information used to determine the current status of a {@link Credential}.
*/
class RevocationTimeframeStatus {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RevocationTimeframeStatus.prototype);
        obj.__wbg_ptr = ptr;
        RevocationTimeframeStatusFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RevocationTimeframeStatusFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_revocationtimeframestatus_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {RevocationTimeframeStatus}
    */
    clone() {
        const ret = wasm.revocationtimeframestatus_clone(this.__wbg_ptr);
        return RevocationTimeframeStatus.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationtimeframestatus_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {RevocationTimeframeStatus}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationtimeframestatus_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return RevocationTimeframeStatus.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Creates a new `RevocationTimeframeStatus`.
    * @param {string} id
    * @param {number} index
    * @param {Duration} duration
    * @param {Timestamp | undefined} [start_validity]
    */
    constructor(id, index, duration, start_validity) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            _assertClass(duration, Duration);
            var ptr1 = duration.__destroy_into_raw();
            let ptr2 = 0;
            if (!isLikeNone(start_validity)) {
                _assertClass(start_validity, Timestamp);
                ptr2 = start_validity.__destroy_into_raw();
            }
            wasm.revocationtimeframestatus_new(retptr, ptr0, len0, index, ptr1, ptr2);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            RevocationTimeframeStatusFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get startValidityTimeframe value.
    * @returns {Timestamp}
    */
    startValidityTimeframe() {
        const ret = wasm.revocationtimeframestatus_startValidityTimeframe(this.__wbg_ptr);
        return Timestamp.__wrap(ret);
    }
    /**
    * Get endValidityTimeframe value.
    * @returns {Timestamp}
    */
    endValidityTimeframe() {
        const ret = wasm.revocationtimeframestatus_endValidityTimeframe(this.__wbg_ptr);
        return Timestamp.__wrap(ret);
    }
    /**
    * Return the URL fo the `RevocationBitmapStatus`.
    * @returns {string}
    */
    id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationtimeframestatus_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Return the index of the credential in the issuer's revocation bitmap
    * @returns {number | undefined}
    */
    index() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.revocationtimeframestatus_index(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.RevocationTimeframeStatus = RevocationTimeframeStatus;

const SdJwtFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_sdjwt_free(ptr >>> 0, 1));
/**
* Representation of an SD-JWT of the format
* `<Issuer-signed JWT>~<Disclosure 1>~<Disclosure 2>~...~<Disclosure N>~<optional KB-JWT>`.
*/
class SdJwt {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SdJwt.prototype);
        obj.__wbg_ptr = ptr;
        SdJwtFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SdJwtFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sdjwt_free(ptr, 0);
    }
    /**
    * Creates a new `SdJwt` from its components.
    * @param {string} jwt
    * @param {Array<string>} disclosures
    * @param {string | undefined} [key_binding_jwt]
    */
    constructor(jwt, disclosures, key_binding_jwt) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(jwt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(key_binding_jwt) ? 0 : passStringToWasm0(key_binding_jwt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.sdjwt_new(retptr, ptr0, len0, addHeapObject(disclosures), ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            SdJwtFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes the components into the final SD-JWT.
    * @returns {string}
    */
    presentation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdjwt_presentation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Parses an SD-JWT into its components as [`SdJwt`].
    *
    * ## Error
    * Returns `DeserializationError` if parsing fails.
    * @param {string} sd_jwt
    * @returns {SdJwt}
    */
    static parse(sd_jwt) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(sd_jwt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.sdjwt_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return SdJwt.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes the components into the final SD-JWT.
    * @returns {string}
    */
    toString() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdjwt_presentation(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * The JWT part.
    * @returns {string}
    */
    jwt() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdjwt_jwt(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * The disclosures part.
    * @returns {Array<string>}
    */
    disclosures() {
        const ret = wasm.sdjwt_disclosures(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * The optional key binding JWT.
    * @returns {string | undefined}
    */
    keyBindingJwt() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdjwt_keyBindingJwt(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1, 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deep clones the object.
    * @returns {SdJwt}
    */
    clone() {
        const ret = wasm.sdjwt_clone(this.__wbg_ptr);
        return SdJwt.__wrap(ret);
    }
}
module.exports.SdJwt = SdJwt;

const SdJwtCredentialValidatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_sdjwtcredentialvalidator_free(ptr >>> 0, 1));
/**
* A type for decoding and validating {@link Credential}.
*/
class SdJwtCredentialValidator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SdJwtCredentialValidatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sdjwtcredentialvalidator_free(ptr, 0);
    }
    /**
    * Creates a new `SdJwtCredentialValidator`. If a `signatureVerifier` is provided it will be used when
    * verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
    * algorithm will be used.
    * @param {IJwsVerifier} signatureVerifier
    */
    constructor(signatureVerifier) {
        const ret = wasm.sdjwtcredentialvalidator_new(addHeapObject(signatureVerifier));
        this.__wbg_ptr = ret >>> 0;
        SdJwtCredentialValidatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Decodes and validates a `Credential` issued as an SD-JWT. A `DecodedJwtCredential` is returned upon success.
    * The credential is constructed by replacing disclosures following the
    * [`Selective Disclosure for JWTs (SD-JWT)`](https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html) standard.
    *
    * The following properties are validated according to `options`:
    * - the issuer's signature on the JWS,
    * - the expiration date,
    * - the issuance date,
    * - the semantic structure.
    *
    * # Warning
    * * The key binding JWT is not validated. If needed, it must be validated separately using
    * `SdJwtValidator::validate_key_binding_jwt`.
    * * The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
    * trusted. This section contains more information on additional checks that should be carried out before and after
    * calling this method.
    *
    * ## The state of the issuer's DID Document
    * The caller must ensure that `issuer` represents an up-to-date DID Document.
    *
    * ## Properties that are not validated
    *  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
    * `proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
    * These should be manually checked after validation, according to your requirements.
    *
    * # Errors
    * An error is returned whenever a validated condition is not satisfied.
    * @param {SdJwt} sd_jwt
    * @param {CoreDocument | IToCoreDocument} issuer
    * @param {JwtCredentialValidationOptions} options
    * @param {FailFast} fail_fast
    * @returns {DecodedJwtCredential}
    */
    validateCredential(sd_jwt, issuer, options, fail_fast) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(sd_jwt, SdJwt);
            _assertClass(options, JwtCredentialValidationOptions);
            wasm.sdjwtcredentialvalidator_validateCredential(retptr, this.__wbg_ptr, sd_jwt.__wbg_ptr, addBorrowedObject(issuer), options.__wbg_ptr, fail_fast);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJwtCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Decode and verify the JWS signature of a `Credential` issued as an SD-JWT using the DID Document of a trusted
    * issuer and replaces the disclosures.
    *
    * A `DecodedJwtCredential` is returned upon success.
    *
    * # Warning
    * The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
    *
    * ## Proofs
    *  Only the JWS signature is verified. If the `Credential` contains a `proof` property this will not be verified
    * by this method.
    *
    * # Errors
    * * If the issuer' URL cannot be parsed.
    * * If Signature verification fails.
    * * If SD decoding fails.
    * @param {SdJwt} credential
    * @param {Array<CoreDocument | IToCoreDocument>} trustedIssuers
    * @param {JwsVerificationOptions} options
    * @returns {DecodedJwtCredential}
    */
    verifySignature(credential, trustedIssuers, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, SdJwt);
            _assertClass(options, JwsVerificationOptions);
            wasm.sdjwtcredentialvalidator_verifySignature(retptr, this.__wbg_ptr, credential.__wbg_ptr, addBorrowedObject(trustedIssuers), options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return DecodedJwtCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Validates a Key Binding JWT (KB-JWT) according to `https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-key-binding-jwt`.
    * The Validation process includes:
    *   * Signature validation using public key materials defined in the `holder` document.
    *   * `typ` value in KB-JWT header.
    *   * `sd_hash` claim value in the KB-JWT claim.
    *   * Optional `nonce`, `aud` and issuance date validation.
    * @param {SdJwt} sdJwt
    * @param {CoreDocument | IToCoreDocument} holder
    * @param {KeyBindingJWTValidationOptions} options
    * @returns {KeyBindingJwtClaims}
    */
    validateKeyBindingJwt(sdJwt, holder, options) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(sdJwt, SdJwt);
            _assertClass(options, KeyBindingJWTValidationOptions);
            wasm.sdjwtcredentialvalidator_validateKeyBindingJwt(retptr, this.__wbg_ptr, sdJwt.__wbg_ptr, addBorrowedObject(holder), options.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return KeyBindingJwtClaims.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.SdJwtCredentialValidator = SdJwtCredentialValidator;

const SdObjectDecoderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_sdobjectdecoder_free(ptr >>> 0, 1));
/**
* Substitutes digests in an SD-JWT object by their corresponding plaintext values provided by disclosures.
*/
class SdObjectDecoder {

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SdObjectDecoderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sdobjectdecoder_free(ptr, 0);
    }
    /**
    * Creates a new `SdObjectDecoder` with `sha-256` hasher.
    */
    constructor() {
        const ret = wasm.sdobjectdecoder_new();
        this.__wbg_ptr = ret >>> 0;
        SdObjectDecoderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Decodes an SD-JWT `object` containing by Substituting the digests with their corresponding
    * plaintext values provided by `disclosures`.
    *
    * ## Notes
    * * Claims like `exp` or `iat` are not validated in the process of decoding.
    * * `_sd_alg` property will be removed if present.
    * @param {Record<string, any>} object
    * @param {Array<string>} disclosures
    * @returns {Record<string, any>}
    */
    decode(object, disclosures) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectdecoder_decode(retptr, this.__wbg_ptr, addHeapObject(object), addHeapObject(disclosures));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.SdObjectDecoder = SdObjectDecoder;

const SdObjectEncoderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_sdobjectencoder_free(ptr >>> 0, 1));
/**
* Transforms a JSON object into an SD-JWT object by substituting selected values
* with their corresponding disclosure digests.
*
* Note: digests are created using the sha-256 algorithm.
*/
class SdObjectEncoder {

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SdObjectEncoderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sdobjectencoder_free(ptr, 0);
    }
    /**
    * Creates a new `SdObjectEncoder` with `sha-256` hash function.
    * @param {any} object
    */
    constructor(object) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectencoder_new(retptr, addBorrowedObject(object));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            SdObjectEncoderFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Substitutes a value with the digest of its disclosure.
    * If no salt is provided, the disclosure will be created with a random salt value.
    *
    * `path` indicates the pointer to the value that will be concealed using the syntax of
    * [JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901).
    *
    * For the following object:
    *
    *  ```
    * {
    *   "id": "did:value",
    *   "claim1": {
    *      "abc": true
    *   },
    *   "claim2": ["val_1", "val_2"]
    * }
    * ```
    *
    * Path "/id" conceals `"id": "did:value"`
    * Path "/claim1/abc" conceals `"abc": true`
    * Path "/claim2/0" conceals `val_1`
    * ```
    *
    * ## Errors
    * * `InvalidPath` if pointer is invalid.
    * * `DataTypeMismatch` if existing SD format is invalid.
    * @param {string} path
    * @param {string | undefined} [salt]
    * @returns {Disclosure}
    */
    conceal(path, salt) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(salt) ? 0 : passStringToWasm0(salt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.sdobjectencoder_conceal(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Disclosure.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds the `_sd_alg` property to the top level of the object, with
    * its value set to "sha-256".
    */
    addSdAlgProperty() {
        wasm.sdobjectencoder_addSdAlgProperty(this.__wbg_ptr);
    }
    /**
    * Returns the modified object as a string.
    * @returns {string}
    */
    encodeToString() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectencoder_encodeToString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Returns the modified object as a string.
    * @returns {string}
    */
    toString() {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectencoder_encodeToString(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            var ptr1 = r0;
            var len1 = r1;
            if (r3) {
                ptr1 = 0; len1 = 0;
                throw takeObject(r2);
            }
            deferred2_0 = ptr1;
            deferred2_1 = len1;
            return getStringFromWasm0(ptr1, len1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
    * Returns the modified object.
    * @returns {Record<string, any>}
    */
    encodeToObject() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectencoder_encodeToObject(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the modified object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.sdobjectencoder_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds a decoy digest to the specified path.
    * If path is an empty slice, decoys will be added to the top level.
    * @param {string} path
    * @param {number} number_of_decoys
    */
    addDecoys(path, number_of_decoys) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.sdobjectencoder_addDecoys(retptr, this.__wbg_ptr, ptr0, len0, number_of_decoys);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.SdObjectEncoder = SdObjectEncoder;

const SelectiveDisclosurePresentationFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_selectivedisclosurepresentation_free(ptr >>> 0, 1));
/**
* Used to construct a JwpPresentedBuilder and handle the selective disclosure of attributes
* - @context MUST NOT be blinded
* - id MUST be blinded
* - type MUST NOT be blinded
* - issuer MUST NOT be blinded
* - issuanceDate MUST be blinded (if Timeframe Revocation mechanism is used)
* - expirationDate MUST be blinded (if Timeframe Revocation mechanism is used)
* - credentialSubject (User have to choose which attribute must be blinded)
* - credentialSchema MUST NOT be blinded
* - credentialStatus MUST NOT be blinded
* - refreshService MUST NOT be blinded (probably will be used for Timeslot Revocation mechanism)
* - termsOfUse NO reason to use it in ZK VC (will be in any case blinded)
* - evidence (User have to choose which attribute must be blinded)
*/
class SelectiveDisclosurePresentation {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SelectiveDisclosurePresentationFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_selectivedisclosurepresentation_free(ptr, 0);
    }
    /**
    * Initialize a presentation starting from an Issued JWP.
    * The properties `jti`, `nbf`, `issuanceDate`, `expirationDate` and `termsOfUse` are concealed by default.
    * @param {JwpIssued} issued_jwp
    */
    constructor(issued_jwp) {
        _assertClass(issued_jwp, JwpIssued);
        var ptr0 = issued_jwp.__destroy_into_raw();
        const ret = wasm.selectivedisclosurepresentation_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        SelectiveDisclosurePresentationFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Selectively disclose "credentialSubject" attributes.
    * # Example
    * ```
    * {
    *     "id": 1234,
    *     "name": "Alice",
    *     "mainCourses": ["Object-oriented Programming", "Mathematics"],
    *     "degree": {
    *         "type": "BachelorDegree",
    *         "name": "Bachelor of Science and Arts",
    *     },
    *     "GPA": "4.0",
    * }
    * ```
    * If you want to undisclose for example the Mathematics course and the name of the degree:
    * ```
    * undisclose_subject("mainCourses[1]");
    * undisclose_subject("degree.name");
    * ```
    * @param {string} path
    */
    concealInSubject(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.selectivedisclosurepresentation_concealInSubject(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Undiscloses "evidence" attributes.
    * @param {string} path
    */
    concealInEvidence(path) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.selectivedisclosurepresentation_concealInEvidence(retptr, this.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets presentation protected header.
    * @param {PresentationProtectedHeader} header
    */
    setPresentationHeader(header) {
        _assertClass(header, PresentationProtectedHeader);
        var ptr0 = header.__destroy_into_raw();
        wasm.selectivedisclosurepresentation_setPresentationHeader(this.__wbg_ptr, ptr0);
    }
}
module.exports.SelectiveDisclosurePresentation = SelectiveDisclosurePresentation;

const ServiceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_service_free(ptr >>> 0, 1));
/**
* A DID Document Service used to enable trusted interactions associated with a DID subject.
*/
class Service {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Service.prototype);
        obj.__wbg_ptr = ptr;
        ServiceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ServiceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_service_free(ptr, 0);
    }
    /**
    * @param {IService} service
    */
    constructor(service) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.service_new(retptr, addHeapObject(service));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            ServiceFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the {@link Service} id.
    * @returns {DIDUrl}
    */
    id() {
        const ret = wasm.service_id(this.__wbg_ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Returns a copy of the {@link Service} type.
    * @returns {Array<string>}
    */
    type() {
        const ret = wasm.service_type(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns a copy of the {@link Service} endpoint.
    * @returns {string | string[] | Map<string, string[]>}
    */
    serviceEndpoint() {
        const ret = wasm.service_serviceEndpoint(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Returns a copy of the custom properties on the {@link Service}.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.service_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.service_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Service}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.service_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Service.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {Service}
    */
    clone() {
        const ret = wasm.service_clone(this.__wbg_ptr);
        return Service.__wrap(ret);
    }
}
module.exports.Service = Service;

const StatusList2021Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_statuslist2021_free(ptr >>> 0, 1));
/**
* StatusList2021 data structure as described in [W3C's VC status list 2021](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/).
*/
class StatusList2021 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StatusList2021.prototype);
        obj.__wbg_ptr = ptr;
        StatusList2021Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatusList2021Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statuslist2021_free(ptr, 0);
    }
    /**
    * Deep clones the object.
    * @returns {StatusList2021}
    */
    clone() {
        const ret = wasm.statuslist2021_clone(this.__wbg_ptr);
        return StatusList2021.__wrap(ret);
    }
    /**
    * Creates a new {@link StatusList2021} of `size` entries.
    * @param {number | undefined} [size]
    */
    constructor(size) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021_new(retptr, !isLikeNone(size), isLikeNone(size) ? 0 : size);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            StatusList2021Finalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the number of entries in this {@link StatusList2021}.
    * @returns {number}
    */
    len() {
        const ret = wasm.statuslist2021_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Returns whether the entry at `index` is set.
    * @param {number} index
    * @returns {boolean}
    */
    get(index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021_get(retptr, this.__wbg_ptr, index);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the value of the `index`-th entry.
    * @param {number} index
    * @param {boolean} value
    */
    set(index, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021_set(retptr, this.__wbg_ptr, index, value);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Encodes this {@link StatusList2021} into its compressed
    * base64 string representation.
    * @returns {string}
    */
    intoEncodedStr() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021_intoEncodedStr(retptr, ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Attempts to decode a {@link StatusList2021} from a string.
    * @param {string} s
    * @returns {StatusList2021}
    */
    static fromEncodedStr(s) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.statuslist2021_fromEncodedStr(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.StatusList2021 = StatusList2021;

const StatusList2021CredentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_statuslist2021credential_free(ptr >>> 0, 1));
/**
* A parsed [StatusList2021Credential](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021credential).
*/
class StatusList2021Credential {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StatusList2021Credential.prototype);
        obj.__wbg_ptr = ptr;
        StatusList2021CredentialFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatusList2021CredentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statuslist2021credential_free(ptr, 0);
    }
    /**
    * Creates a new {@link StatusList2021Credential}.
    * @param {Credential} credential
    */
    constructor(credential) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            var ptr0 = credential.__destroy_into_raw();
            wasm.statuslist2021credential_new(retptr, ptr0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            StatusList2021CredentialFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string}
    */
    id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021credential_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Sets the given credential's status using the `index`-th entry of this status list.
    * Returns the created `credentialStatus`.
    * @param {Credential} credential
    * @param {number} index
    * @param {boolean} revoked_or_suspended
    * @returns {StatusList2021Entry}
    */
    setCredentialStatus(credential, index, revoked_or_suspended) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(credential, Credential);
            wasm.statuslist2021credential_setCredentialStatus(retptr, this.__wbg_ptr, credential.__wbg_ptr, index, revoked_or_suspended);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021Entry.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns the {@link StatusPurpose} of this {@link StatusList2021Credential}.
    * @returns {StatusPurpose}
    */
    purpose() {
        const ret = wasm.statuslist2021credential_purpose(this.__wbg_ptr);
        return ret;
    }
    /**
    * Returns the state of the `index`-th entry, if any.
    * @param {number} index
    * @returns {CredentialStatus}
    */
    entry(index) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021credential_entry(retptr, this.__wbg_ptr, index);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return r0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {StatusList2021Credential}
    */
    clone() {
        const ret = wasm.statuslist2021credential_clone(this.__wbg_ptr);
        return StatusList2021Credential.__wrap(ret);
    }
    /**
    * @param {any} json
    * @returns {StatusList2021Credential}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021credential_fromJSON(retptr, addHeapObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021Credential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021credential_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.StatusList2021Credential = StatusList2021Credential;

const StatusList2021CredentialBuilderFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_statuslist2021credentialbuilder_free(ptr >>> 0, 1));
/**
* Builder type to construct valid {@link StatusList2021Credential} istances.
*/
class StatusList2021CredentialBuilder {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StatusList2021CredentialBuilder.prototype);
        obj.__wbg_ptr = ptr;
        StatusList2021CredentialBuilderFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatusList2021CredentialBuilderFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statuslist2021credentialbuilder_free(ptr, 0);
    }
    /**
    * Creates a new {@link StatusList2021CredentialBuilder}.
    * @param {StatusList2021 | undefined} [status_list]
    */
    constructor(status_list) {
        let ptr0 = 0;
        if (!isLikeNone(status_list)) {
            _assertClass(status_list, StatusList2021);
            ptr0 = status_list.__destroy_into_raw();
        }
        const ret = wasm.statuslist2021credentialbuilder_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        StatusList2021CredentialBuilderFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Sets the purpose of the {@link StatusList2021Credential} that is being created.
    * @param {StatusPurpose} purpose
    * @returns {StatusList2021CredentialBuilder}
    */
    purpose(purpose) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.statuslist2021credentialbuilder_purpose(ptr, purpose);
        return StatusList2021CredentialBuilder.__wrap(ret);
    }
    /**
    * Sets `credentialSubject.id`.
    * @param {string} id
    * @returns {StatusList2021CredentialBuilder}
    */
    subjectId(id) {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.statuslist2021credentialbuilder_subjectId(retptr, ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021CredentialBuilder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the expiration date of the credential.
    * @param {Timestamp} time
    * @returns {StatusList2021CredentialBuilder}
    */
    expirationDate(time) {
        const ptr = this.__destroy_into_raw();
        _assertClass(time, Timestamp);
        var ptr0 = time.__destroy_into_raw();
        const ret = wasm.statuslist2021credentialbuilder_expirationDate(ptr, ptr0);
        return StatusList2021CredentialBuilder.__wrap(ret);
    }
    /**
    * Sets the issuer of the credential.
    * @param {string} issuer
    * @returns {StatusList2021CredentialBuilder}
    */
    issuer(issuer) {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(issuer, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.statuslist2021credentialbuilder_issuer(retptr, ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021CredentialBuilder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Sets the context of the credential.
    * @param {string} context
    * @returns {StatusList2021CredentialBuilder}
    */
    context(context) {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(context, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.statuslist2021credentialbuilder_context(retptr, ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021CredentialBuilder.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds a credential type.
    * @param {string} t
    * @returns {StatusList2021CredentialBuilder}
    */
    type(t) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(t, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.statuslist2021credentialbuilder_type(ptr, ptr0, len0);
        return StatusList2021CredentialBuilder.__wrap(ret);
    }
    /**
    * Adds a credential's proof.
    * @param {Proof} proof
    * @returns {StatusList2021CredentialBuilder}
    */
    proof(proof) {
        const ptr = this.__destroy_into_raw();
        _assertClass(proof, Proof);
        var ptr0 = proof.__destroy_into_raw();
        const ret = wasm.statuslist2021credentialbuilder_proof(ptr, ptr0);
        return StatusList2021CredentialBuilder.__wrap(ret);
    }
    /**
    * Attempts to build a valid {@link StatusList2021Credential} with the previously provided data.
    * @returns {StatusList2021Credential}
    */
    build() {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021credentialbuilder_build(retptr, ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021Credential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
module.exports.StatusList2021CredentialBuilder = StatusList2021CredentialBuilder;

const StatusList2021EntryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_statuslist2021entry_free(ptr >>> 0, 1));
/**
* [StatusList2021Entry](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry) implementation.
*/
class StatusList2021Entry {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StatusList2021Entry.prototype);
        obj.__wbg_ptr = ptr;
        StatusList2021EntryFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatusList2021EntryFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statuslist2021entry_free(ptr, 0);
    }
    /**
    * Creates a new {@link StatusList2021Entry}.
    * @param {string} status_list
    * @param {StatusPurpose} purpose
    * @param {number} index
    * @param {string | undefined} [id]
    */
    constructor(status_list, purpose, index, id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(status_list, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            var ptr1 = isLikeNone(id) ? 0 : passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.statuslist2021entry_new(retptr, ptr0, len0, purpose, index, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            StatusList2021EntryFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns this `credentialStatus`'s `id`.
    * @returns {string}
    */
    id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021entry_id(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the purpose of this entry.
    * @returns {StatusPurpose}
    */
    purpose() {
        const ret = wasm.statuslist2021entry_purpose(this.__wbg_ptr);
        return ret;
    }
    /**
    * Returns the index of this entry.
    * @returns {number}
    */
    index() {
        const ret = wasm.statuslist2021entry_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Returns the referenced {@link StatusList2021Credential}'s url.
    * @returns {string}
    */
    statusListCredential() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021entry_statusListCredential(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Downcasts {@link this} to {@link Status}
    * @returns {Status}
    */
    toStatus() {
        try {
            const ptr = this.__destroy_into_raw();
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021entry_toStatus(retptr, ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deep clones the object.
    * @returns {StatusList2021Entry}
    */
    clone() {
        const ret = wasm.statuslist2021entry_clone(this.__wbg_ptr);
        return StatusList2021Entry.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021entry_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {StatusList2021Entry}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.statuslist2021entry_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return StatusList2021Entry.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.StatusList2021Entry = StatusList2021Entry;

const StorageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_storage_free(ptr >>> 0, 1));
/**
* A type wrapping a `JwkStorage` and `KeyIdStorage` that should always be used together when
* working with storage backed DID documents.
*/
class Storage {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StorageFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_storage_free(ptr, 0);
    }
    /**
    * Constructs a new `Storage`.
    * @param {JwkStorage} jwkStorage
    * @param {KeyIdStorage} keyIdStorage
    */
    constructor(jwkStorage, keyIdStorage) {
        const ret = wasm.storage_new(addHeapObject(jwkStorage), addHeapObject(keyIdStorage));
        this.__wbg_ptr = ret >>> 0;
        StorageFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Obtain the wrapped `KeyIdStorage`.
    * @returns {KeyIdStorage}
    */
    keyIdStorage() {
        const ret = wasm.storage_keyIdStorage(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Obtain the wrapped `JwkStorage`.
    * @returns {JwkStorage}
    */
    keyStorage() {
        const ret = wasm.storage_keyStorage(this.__wbg_ptr);
        return takeObject(ret);
    }
}
module.exports.Storage = Storage;

const TimestampFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_timestamp_free(ptr >>> 0, 1));
/**
*/
class Timestamp {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Timestamp.prototype);
        obj.__wbg_ptr = ptr;
        TimestampFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TimestampFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_timestamp_free(ptr, 0);
    }
    /**
    * Creates a new {@link Timestamp} with the current date and time.
    */
    constructor() {
        const ret = wasm.timestamp_new();
        this.__wbg_ptr = ret >>> 0;
        TimestampFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Parses a {@link Timestamp} from the provided input string.
    * @param {string} input
    * @returns {Timestamp}
    */
    static parse(input) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.timestamp_parse(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Timestamp.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Creates a new {@link Timestamp} with the current date and time.
    * @returns {Timestamp}
    */
    static nowUTC() {
        const ret = wasm.timestamp_new();
        return Timestamp.__wrap(ret);
    }
    /**
    * Returns the {@link Timestamp} as an RFC 3339 `String`.
    * @returns {string}
    */
    toRFC3339() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.timestamp_toRFC3339(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Computes `self + duration`
    *
    * Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
    * @param {Duration} duration
    * @returns {Timestamp | undefined}
    */
    checkedAdd(duration) {
        _assertClass(duration, Duration);
        const ret = wasm.timestamp_checkedAdd(this.__wbg_ptr, duration.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * Computes `self - duration`
    *
    * Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
    * @param {Duration} duration
    * @returns {Timestamp | undefined}
    */
    checkedSub(duration) {
        _assertClass(duration, Duration);
        const ret = wasm.timestamp_checkedSub(this.__wbg_ptr, duration.__wbg_ptr);
        return ret === 0 ? undefined : Timestamp.__wrap(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.timestamp_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {Timestamp}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.timestamp_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return Timestamp.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
}
module.exports.Timestamp = Timestamp;

const UnknownCredentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_unknowncredential_free(ptr >>> 0, 1));
/**
*/
class UnknownCredential {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(UnknownCredential.prototype);
        obj.__wbg_ptr = ptr;
        UnknownCredentialFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UnknownCredentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_unknowncredential_free(ptr, 0);
    }
    /**
    * Returns a {@link Jwt} if the credential is of type string, `undefined` otherwise.
    * @returns {Jwt | undefined}
    */
    tryIntoJwt() {
        const ret = wasm.unknowncredential_tryIntoJwt(this.__wbg_ptr);
        return ret === 0 ? undefined : Jwt.__wrap(ret);
    }
    /**
    * Returns a {@link Credential} if the credential is of said type, `undefined` otherwise.
    * @returns {Credential | undefined}
    */
    tryIntoCredential() {
        const ret = wasm.unknowncredential_tryIntoCredential(this.__wbg_ptr);
        return ret === 0 ? undefined : Credential.__wrap(ret);
    }
    /**
    * Returns the contained value as an Object, if it can be converted, `undefined` otherwise.
    * @returns {Record<string, any> | undefined}
    */
    tryIntoRaw() {
        const ret = wasm.unknowncredential_tryIntoRaw(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.unknowncredential_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {UnknownCredential}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.unknowncredential_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return UnknownCredential.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {UnknownCredential}
    */
    clone() {
        const ret = wasm.unknowncredential_clone(this.__wbg_ptr);
        return UnknownCredential.__wrap(ret);
    }
}
module.exports.UnknownCredential = UnknownCredential;

const VerificationMethodFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => { }, unregister: () => { } }
    : new FinalizationRegistry(ptr => wasm.__wbg_verificationmethod_free(ptr >>> 0, 1));
/**
* A DID Document Verification Method.
*/
class VerificationMethod {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(VerificationMethod.prototype);
        obj.__wbg_ptr = ptr;
        VerificationMethodFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    toJSON() {
        return {
        };
    }

    toString() {
        return JSON.stringify(this);
    }

    [inspect.custom]() {
        return Object.assign(Object.create({ constructor: this.constructor }), this.toJSON());
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VerificationMethodFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_verificationmethod_free(ptr, 0);
    }
    /**
    * Creates a new {@link VerificationMethod} from the given `did` and {@link Jwk}. If `fragment` is not given
    * the `kid` value of the given `key` will be used, if present, otherwise an error is returned.
    *
    * ### Recommendations
    * The following recommendations are essentially taken from the `publicKeyJwk` description from the [DID specification](https://www.w3.org/TR/did-core/#dfn-publickeyjwk):
    * - It is recommended that verification methods that use `Jwks` to represent their public keys use the value of
    *   `kid` as their fragment identifier. This is
    * done automatically if `None` is passed in as the fragment.
    * - It is recommended that {@link Jwk} kid values are set to the public key fingerprint.
    * @param {CoreDID | IToCoreDID} did
    * @param {Jwk} key
    * @param {string | undefined} [fragment]
    * @returns {VerificationMethod}
    */
    static newFromJwk(did, key, fragment) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(key, Jwk);
            var ptr0 = isLikeNone(fragment) ? 0 : passStringToWasm0(fragment, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            wasm.verificationmethod_newFromJwk(retptr, addBorrowedObject(did), key.__wbg_ptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Create a custom {@link VerificationMethod}.
    * @param {DIDUrl} id
    * @param {CoreDID} controller
    * @param {MethodType} type_
    * @param {MethodData} data
    */
    constructor(id, controller, type_, data) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(id, DIDUrl);
            _assertClass(controller, CoreDID);
            _assertClass(type_, MethodType);
            _assertClass(data, MethodData);
            wasm.verificationmethod_new(retptr, id.__wbg_ptr, controller.__wbg_ptr, type_.__wbg_ptr, data.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            VerificationMethodFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the {@link DIDUrl} of the {@link VerificationMethod}'s `id`.
    * @returns {DIDUrl}
    */
    id() {
        const ret = wasm.verificationmethod_id(this.__wbg_ptr);
        return DIDUrl.__wrap(ret);
    }
    /**
    * Sets the id of the {@link VerificationMethod}.
    * @param {DIDUrl} id
    */
    setId(id) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(id, DIDUrl);
            wasm.verificationmethod_setId(retptr, this.__wbg_ptr, id.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Returns a copy of the `controller` `DID` of the {@link VerificationMethod}.
    * @returns {CoreDID}
    */
    controller() {
        const ret = wasm.verificationmethod_controller(this.__wbg_ptr);
        return CoreDID.__wrap(ret);
    }
    /**
    * Sets the `controller` `DID` of the {@link VerificationMethod} object.
    * @param {CoreDID} did
    */
    setController(did) {
        _assertClass(did, CoreDID);
        wasm.verificationmethod_setController(this.__wbg_ptr, did.__wbg_ptr);
    }
    /**
    * Returns a copy of the {@link VerificationMethod} type.
    * @returns {MethodType}
    */
    type() {
        const ret = wasm.verificationmethod_type(this.__wbg_ptr);
        return MethodType.__wrap(ret);
    }
    /**
    * Sets the {@link VerificationMethod} type.
    * @param {MethodType} type_
    */
    setType(type_) {
        _assertClass(type_, MethodType);
        wasm.verificationmethod_setType(this.__wbg_ptr, type_.__wbg_ptr);
    }
    /**
    * Returns a copy of the {@link VerificationMethod} public key data.
    * @returns {MethodData}
    */
    data() {
        const ret = wasm.verificationmethod_data(this.__wbg_ptr);
        return MethodData.__wrap(ret);
    }
    /**
    * Sets {@link VerificationMethod} public key data.
    * @param {MethodData} data
    */
    setData(data) {
        _assertClass(data, MethodData);
        wasm.verificationmethod_setData(this.__wbg_ptr, data.__wbg_ptr);
    }
    /**
    * Get custom properties of the Verification Method.
    * @returns {Map<string, any>}
    */
    properties() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.verificationmethod_properties(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Adds a custom property to the Verification Method.
    * If the value is set to `null`, the custom property will be removed.
    *
    * ### WARNING
    * This method can overwrite existing properties like `id` and result
    * in an invalid Verification Method.
    * @param {string} key
    * @param {any} value
    */
    setPropertyUnchecked(key, value) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.verificationmethod_setPropertyUnchecked(retptr, this.__wbg_ptr, ptr0, len0, addBorrowedObject(value));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Serializes this to a JSON object.
    * @returns {any}
    */
    toJSON() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.verificationmethod_toJSON(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Deserializes an instance from a JSON object.
    * @param {any} json
    * @returns {VerificationMethod}
    */
    static fromJSON(json) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.verificationmethod_fromJSON(retptr, addBorrowedObject(json));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return VerificationMethod.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Deep clones the object.
    * @returns {VerificationMethod}
    */
    clone() {
        const ret = wasm.verificationmethod_clone(this.__wbg_ptr);
        return VerificationMethod.__wrap(ret);
    }
}
module.exports.VerificationMethod = VerificationMethod;

module.exports.__wbg_service_new = function (arg0) {
    const ret = Service.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_object_drop_ref = function (arg0) {
    takeObject(arg0);
};

module.exports.__wbg_verificationmethod_new = function (arg0) {
    const ret = VerificationMethod.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_jwt_new = function (arg0) {
    const ret = Jwt.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_unknowncredential_new = function (arg0) {
    const ret = UnknownCredential.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_jpt_new = function (arg0) {
    const ret = Jpt.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_iotadocument_new = function (arg0) {
    const ret = IotaDocument.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_coredid_new = function (arg0) {
    const ret = CoreDID.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_jws_new = function (arg0) {
    const ret = Jws.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_iotadid_new = function (arg0) {
    const ret = IotaDID.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_didurl_new = function (arg0) {
    const ret = DIDUrl.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_jwkgenoutput_new = function (arg0) {
    const ret = JwkGenOutput.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_payloadentry_unwrap = function (arg0) {
    const ret = PayloadEntry.__unwrap(takeObject(arg0));
    return ret;
};

module.exports.__wbindgen_string_new = function (arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbindgen_object_clone_ref = function (arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_error_new = function (arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_generate_7a89d14c93c6da18 = function (arg0, arg1, arg2, arg3, arg4) {
    let deferred0_0;
    let deferred0_1;
    let deferred1_0;
    let deferred1_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        deferred1_0 = arg3;
        deferred1_1 = arg4;
        const ret = getObject(arg0).generate(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
};

module.exports.__wbg_insert_da8b50c74383af0d = function (arg0, arg1) {
    const ret = getObject(arg0).insert(Jwk.__wrap(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_sign_421bfbd38dd4553a = function (arg0, arg1, arg2, arg3, arg4, arg5) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        var v1 = getArrayU8FromWasm0(arg3, arg4).slice();
        wasm.__wbindgen_free(arg3, arg4 * 1, 1);
        const ret = getObject(arg0).sign(getStringFromWasm0(arg1, arg2), v1, Jwk.__wrap(arg5));
        return addHeapObject(ret);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_delete_2ee32e53cb78797f = function (arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        const ret = getObject(arg0).delete(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_getkey_7593da5ee7e5f5f0 = function (arg0, arg1, arg2) {
    const ret = getObject(arg0)._get_key(getStringFromWasm0(arg1, arg2));
    let ptr1 = 0;
    if (!isLikeNone(ret)) {
        _assertClass(ret, Jwk);
        ptr1 = ret.__destroy_into_raw();
    }
    return ptr1;
};

module.exports.__wbindgen_is_string = function (arg0) {
    const ret = typeof (getObject(arg0)) === 'string';
    return ret;
};

module.exports.__wbg_client_9b83f75e2726d87f = function (arg0) {
    const ret = getObject(arg0).client;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

module.exports.__wbg_handlers_faa84e527a39cdf8 = function (arg0) {
    const ret = getObject(arg0).handlers;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

module.exports.__wbindgen_json_parse = function (arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbindgen_json_serialize = function (arg0, arg1) {
    const obj = getObject(arg1);
    const ret = JSON.stringify(obj === undefined ? null : obj);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbg_getAliasOutput_cec17fa4c27f521a = function (arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        const ret = getObject(arg0).getAliasOutput(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_getProtocolParameters_c2d053891e679b55 = function (arg0) {
    const ret = getObject(arg0).getProtocolParameters();
    return addHeapObject(ret);
};

module.exports.__wbg_insertKeyId_99b1ef8ced0ab42e = function (arg0, arg1, arg2, arg3) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg2;
        deferred0_1 = arg3;
        const ret = getObject(arg0).insertKeyId(MethodDigest.__wrap(arg1), getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_getKeyId_5b3b70e12f7e53aa = function (arg0, arg1) {
    const ret = getObject(arg0).getKeyId(MethodDigest.__wrap(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_deleteKeyId_e47ce8f9b5a1d684 = function (arg0, arg1) {
    const ret = getObject(arg0).deleteKeyId(MethodDigest.__wrap(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_verify_0b54d052c343c289 = function () {
    return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg1;
            deferred0_1 = arg2;
            var v1 = getArrayU8FromWasm0(arg3, arg4).slice();
            wasm.__wbindgen_free(arg3, arg4 * 1, 1);
            var v2 = getArrayU8FromWasm0(arg5, arg6).slice();
            wasm.__wbindgen_free(arg5, arg6 * 1, 1);
            getObject(arg0).verify(getStringFromWasm0(arg1, arg2), v1, v2, Jwk.__wrap(arg7));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    }, arguments)
};

module.exports.__wbindgen_string_get = function (arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof (obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbg_maybeGetIotaDocumentInternal_fd250ef4074a0c1e = function (arg0) {
    const ret = _maybeGetIotaDocumentInternal(getObject(arg0));
    let ptr1 = 0;
    if (!isLikeNone(ret)) {
        _assertClass(ret, IotaDocument);
        ptr1 = ret.__destroy_into_raw();
    }
    return ptr1;
};

module.exports.__wbg_getCoreDocumentInternal_e937cd8649fc1d76 = function (arg0) {
    const ret = _getCoreDocumentInternal(getObject(arg0));
    _assertClass(ret, CoreDocument);
    var ptr1 = ret.__destroy_into_raw();
    return ptr1;
};

module.exports.__wbg_id_81f123397a43f160 = function (arg0) {
    const ret = getObject(arg0).id;
    return addHeapObject(ret);
};

module.exports.__wbg_type_a6d92648720e7f8b = function (arg0) {
    const ret = getObject(arg0).type;
    return addHeapObject(ret);
};

module.exports.__wbg_serviceEndpoint_3133f0c11b566fad = function (arg0) {
    const ret = getObject(arg0).serviceEndpoint;
    return addHeapObject(ret);
};

module.exports.__wbg_properties_4246821a1cd87560 = function (arg0) {
    const ret = getObject(arg0).properties;
    return addHeapObject(ret);
};

module.exports.__wbg_getCoreDidCloneInternal_9d05a6ed9c0bc653 = function (arg0) {
    const ret = _getCoreDidCloneInternal(getObject(arg0));
    _assertClass(ret, CoreDID);
    var ptr1 = ret.__destroy_into_raw();
    return ptr1;
};

module.exports.__wbg_new_abda76e883ba8a5f = function () {
    const ret = new Error();
    return addHeapObject(ret);
};

module.exports.__wbg_stack_658279fe44541cf6 = function (arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbg_error_f851667af71bcfc6 = function (arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_queueMicrotask_12a30234db4045d3 = function (arg0) {
    queueMicrotask(getObject(arg0));
};

module.exports.__wbg_queueMicrotask_48421b3cc9052b68 = function (arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
};

module.exports.__wbindgen_is_function = function (arg0) {
    const ret = typeof (getObject(arg0)) === 'function';
    return ret;
};

module.exports.__wbindgen_cb_drop = function (arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

module.exports.__wbg_crypto_1d1f22824a6a080c = function (arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

module.exports.__wbindgen_is_object = function (arg0) {
    const val = getObject(arg0);
    const ret = typeof (val) === 'object' && val !== null;
    return ret;
};

module.exports.__wbg_process_4a72847cc503995b = function (arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};

module.exports.__wbg_versions_f686565e586dd935 = function (arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};

module.exports.__wbg_node_104a2ff8d6ea03a2 = function (arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};

module.exports.__wbg_require_cca90b1a94a0255b = function () {
    return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_msCrypto_eb05e62b530a1508 = function (arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

module.exports.__wbg_randomFillSync_5c9c955aa56b6049 = function () {
    return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments)
};

module.exports.__wbg_getRandomValues_3aa56aa6edec874c = function () {
    return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments)
};

module.exports.__wbg_get_3baa728f9d58d3f6 = function (arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};

module.exports.__wbg_length_ae22078168b726f5 = function (arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

module.exports.__wbg_new_a220cf903aa02ca2 = function () {
    const ret = new Array();
    return addHeapObject(ret);
};

module.exports.__wbg_newnoargs_76313bd6ff35d0f2 = function (arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_new_8608a2b51a5f6737 = function () {
    const ret = new Map();
    return addHeapObject(ret);
};

module.exports.__wbg_next_f9cb570345655b9a = function () {
    return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_done_bfda7aa8f252b39f = function (arg0) {
    const ret = getObject(arg0).done;
    return ret;
};

module.exports.__wbg_value_6d39332ab4788d86 = function (arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
};

module.exports.__wbg_call_1084a111329e68ce = function () {
    return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_self_3093d5d1f7bcb682 = function () {
    return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_window_3bcfc4d31bc012f8 = function () {
    return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_globalThis_86b222e13bdf32ed = function () {
    return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_global_e5a3fe56f8be9485 = function () {
    return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbindgen_is_undefined = function (arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

module.exports.__wbg_from_0791d740a9d37830 = function (arg0) {
    const ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_isArray_8364a5371e9737d8 = function (arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
};

module.exports.__wbg_push_37c89022f34c01ca = function (arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

module.exports.__wbg_instanceof_Error_69bde193b0cc95e3 = function (arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Error;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_new_796382978dfd4fb0 = function (arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_setname_ef058a4c6ff7e6d4 = function (arg0, arg1, arg2) {
    getObject(arg0).name = getStringFromWasm0(arg1, arg2);
};

module.exports.__wbg_toString_9d18e102ca933e68 = function (arg0) {
    const ret = getObject(arg0).toString();
    return addHeapObject(ret);
};

module.exports.__wbg_call_89af060b4e1523f2 = function () {
    return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_instanceof_Map_763ce0e95960d55e = function (arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Map;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_get_5a402b270e32a550 = function (arg0, arg1) {
    const ret = getObject(arg0).get(getObject(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_set_49185437f0ab06f8 = function (arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_keys_2a4853eb6ff523cf = function (arg0) {
    const ret = getObject(arg0).keys();
    return addHeapObject(ret);
};

module.exports.__wbg_now_b7a162010a9e75b4 = function () {
    const ret = Date.now();
    return ret;
};

module.exports.__wbg_fromEntries_623a5958a8dd4673 = function () {
    return handleError(function (arg0) {
        const ret = Object.fromEntries(getObject(arg0));
        return addHeapObject(ret);
    }, arguments)
};

module.exports.__wbg_new_b85e72ed1bfd57f9 = function (arg0, arg1) {
    try {
        var state0 = { a: arg0, b: arg1 };
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_813(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return addHeapObject(ret);
    } finally {
        state0.a = state0.b = 0;
    }
};

module.exports.__wbg_resolve_570458cb99d56a43 = function (arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_then_95e6edc0f89b73b1 = function (arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_then_876bb3c633745cc6 = function (arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_buffer_b7b08af79b0b0974 = function (arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

module.exports.__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9 = function (arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

module.exports.__wbg_new_ea1883e1e5e86686 = function (arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_set_d1e79e2388520f18 = function (arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

module.exports.__wbg_length_8339fcf5d8ecd12e = function (arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

module.exports.__wbg_instanceof_Uint8Array_247a91427532499e = function (arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Uint8Array;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_newwithlength_ec548f448387c968 = function (arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

module.exports.__wbg_subarray_7c2e3576afe181d1 = function (arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_debug_string = function (arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbindgen_throw = function (arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_memory = function () {
    const ret = wasm.memory;
    return addHeapObject(ret);
};

module.exports.__wbindgen_closure_wrapper4915 = function (arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 557, __wbg_adapter_32);
    return addHeapObject(ret);
};

const path = require('path').join(__dirname, 'identity_wasm_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

wasm.__wbindgen_start();

