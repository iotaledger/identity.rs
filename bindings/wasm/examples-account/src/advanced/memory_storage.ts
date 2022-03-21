// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ChainState,
    DID,
    Ed25519,
    IdentityState,
    KeyLocation,
    KeyPair,
    KeyType,
    MethodType,
    Ed25519PrivateKey,
    Signature,
} from '../../../node/identity_wasm.js';

import type { Storage } from '../../../node/identity_wasm.js';

// TODO: add thorough comments explaining what this is and how to use it with an Account.
export class MemStore implements Storage {
    // TODO: check if map key comparison works as-expected.
    //       I.e. does a parsed/deserialized DID map to the same DID object?
    private _chainStates: Map<string, ChainState>;
    private _states: Map<string, IdentityState>;
    private _vaults: Map<string, Map<string, KeyPair>>;

    constructor() {
        this._chainStates = new Map();
        this._states = new Map();
        this._vaults = new Map();
    }

    public async setPassword(_encryptionKey: Uint8Array) {}

    public async flushChanges() {}

    public async keyNew(did: DID, keyLocation: KeyLocation) {
        if (keyLocation.method().toString() !== MethodType.Ed25519VerificationKey2018().toString()) {
            throw new Error('Unsupported Method')
        }
        const keyPair = new KeyPair(KeyType.Ed25519);
        const publicKey = keyPair.public();
        const vault = this._vaults.get(did.toString());
        if (vault) {
            vault.set(keyLocation.toString(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.toString(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }
        return publicKey
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string) {
        if (keyLocation.method().toString() !== MethodType.Ed25519VerificationKey2018().toString()) {
            throw new Error('Unsupported Method')
        }
        const secretKey = Ed25519PrivateKey.fromBase58(privateKey);
        const publicKey = secretKey.publicKey();
        const keyPair = KeyPair.fromBase58(KeyType.Ed25519, privateKey, publicKey);
        const vault = this._vaults.get(did.toString());
        if (vault) {
            vault.set(keyLocation.toString(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.toString(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }
        return publicKey
    }

    public async keyExists(did: DID, keyLocation: KeyLocation) {
        const vault = this._vaults.get(did.toString());
        if (vault) {
            return vault.has(keyLocation.toString())
        }
        return false
    }

    public async keyGet(did: DID, keyLocation: KeyLocation) {
        const vault = this._vaults.get(did.toString());
        if (vault) {
            const keyPair = vault.get(keyLocation.toString());
            if (keyPair) {
                return keyPair.public()
            }
            throw new Error('Key location not found')
        }
        throw new Error('DID not found')
    }

    public async keyDel(did: DID, keyLocation: KeyLocation) {
        const vault = this._vaults.get(did.toString());
        if (vault) {
            vault.delete(keyLocation.toString());
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array) {
        const vault = this._vaults.get(did.toString());
        if (!vault) {
            throw new Error('DID not found')
        }
        const keyPair = vault.get(keyLocation.toString());
        if (!keyPair) {
            throw new Error('Key location not found')
        }
        if (keyLocation.method().toString() !== MethodType.Ed25519VerificationKey2018().toString()) {
            throw new Error('Unsupported Method')
        }
        const signature = Ed25519.sign(data, keyPair.private());
        return new Signature(keyPair.public(), signature)
    }

    public async chainState(did: DID) {
        return this._chainStates.get(did.toString());
    }

    public async setChainState(did: DID, chainState: ChainState) {
        this._chainStates.set(did.toString(), chainState);
    }

    public async state(did: DID) {
        return this._states.get(did.toString())
    }

    public async setState(did: DID, identityState: IdentityState) {
        this._states.set(did.toString(), identityState);
    }

    public async purge(did: DID) {
        this._chainStates.delete(did.toString());
        this._states.delete(did.toString());
        this._vaults.delete(did.toString());
    }
}

