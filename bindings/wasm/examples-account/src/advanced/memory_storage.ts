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
    PrivateKey,
    Signature,
} from '../../../node/identity_wasm.js';

import type { Storage } from '../../../node/identity_wasm.js';

// TODO: add thorough comments explaining what this is and how to use it with an Account.
export class MemStore implements Storage {
    // TODO: check if map key comparison works as-expected.
    //       I.e. does a parsed/deserialized DID map to the same DID object?
    private _chainStates: Map<DID, ChainState>;
    private _states: Map<DID, IdentityState>;
    private _vaults: Map<DID, Map<KeyLocation, KeyPair>>;

    constructor() {
        this._chainStates = new Map();
        this._states = new Map();
        this._vaults = new Map();
    }

    public async setPassword(_encryptionKey: Uint8Array): Promise<void> {}

    public async flushChanges(): Promise<void> {}

    public async keyNew(did: DID, keyLocation: KeyLocation): Promise<string> {
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw new Error('Unsuported Method')
        }
        const keyPair: KeyPair = new KeyPair(KeyType.Ed25519);
        const publicKey: string = keyPair.public;
        const vault = this._vaults.get(did);
        if (vault) {
            vault.set(keyLocation, keyPair);
        } else {
            const newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this._vaults.set(did, newVault);
        }
        return publicKey
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string): Promise<string> {
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw new Error('Unsuported Method')
        }
        const secretKey: PrivateKey = PrivateKey.fromBase58String(privateKey);
        const publicKey: string = secretKey.publicKey();
        const keyPair: KeyPair = KeyPair.fromBase58(KeyType.Ed25519, privateKey, publicKey);
        const vault = this._vaults.get(did);
        if (vault) {
            vault.set(keyLocation, keyPair);
        } else {
            const newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this._vaults.set(did, newVault);
        }
        return publicKey
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const vault = this._vaults.get(did);
        if (vault) {
            return vault.has(keyLocation)
        }
        return false
    }

    public async keyGet(did: DID, keyLocation: KeyLocation): Promise<string> {
        const vault = this._vaults.get(did);
        if (vault) {
            const keyPair = vault.get(keyLocation);
            if (keyPair) {
                return keyPair.public
            }
            throw new Error('Key location not found')
        }
        throw new Error('DID not found')
    }

    public async keyDel(did: DID, keyLocation: KeyLocation): Promise<void> {
        const vault = this._vaults.get(did);
        if (vault) {
            vault.delete(keyLocation);
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        const vault = this._vaults.get(did);
        if (!vault) {
            throw new Error('DID not found')
        }
        const keyPair = vault.get(keyLocation);
        if (!keyPair) {
            throw new Error('Key location not found')
        }
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw new Error('Unsuported Method')
        }
        const signature: Uint8Array = Ed25519.sign(data, keyPair.private);
        return new Signature(keyPair.public, signature)
    }

    public async chainState(did: DID): Promise<ChainState> {
        return this._chainStates.get(did);
    }

    public async setChainState(did: DID, chainState: ChainState): Promise<void> {
        this._chainStates.set(did, chainState);
    }

    public async state(did: DID): Promise<IdentityState> {
        return this._states.get(did)
    }

    public async setState(did: DID, identityState: IdentityState): Promise<void> {
        this._states.set(did, identityState);
    }

    public async purge(did: DID): Promise<void> {
        this._chainStates.delete(did);
        this._states.delete(did);
        this._vaults.delete(did);
    }
}

