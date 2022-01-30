// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {ChainState, DID, DIDLease, Ed25519, Generation, IdentityState, KeyLocation, KeyPair, KeyType, MethodType, SecretKey, Signature} from './../../node/identity_wasm.js';

class MemStore {
    private _expand: boolean;
    private _publishedGenerations: Map<DID, Generation>;
    private _didLeases: Map<DID, DIDLease>;
    private _chainStates: Map<DID, ChainState>;
    private _states: Map<DID, IdentityState>;
    private _vaults: Map<DID, Map<KeyLocation, KeyPair>>;

    constructor() {
        this._expand = false;
        this._publishedGenerations = new Map();
        this._didLeases = new Map();
        this._chainStates = new Map();
        this._states = new Map();
        this._vaults = new Map();
    }

    public get expand(): boolean {
        return this._expand;
    }

    public set expand(value: boolean) {
        this._expand = value;
    }

    public get vaults(): Map<DID, Map<KeyLocation, KeyPair>> {
        return this._vaults
    }

    public setPassword(_encryptionKey: Uint8Array): Promise<void> {
        return new Promise<void>((resolve, reject) => {})
    }

    public flushChanges(): Promise<void> {
        return new Promise<void>((resolve, reject) => {})
    }

    public async leaseDid(did: DID): Promise<DIDLease> {
        if (this._didLeases.has(did)) {
            let didLease: DIDLease = this._didLeases.get(did);
            if (didLease.load()) {
                throw 'Identity in Use'
            } 
            didLease.store(true);
            return didLease
        
        }
        let didLease = new DIDLease();
        this._didLeases.set(did, didLease);
        return didLease
    }

    public async keyNew(did: DID, keyLocation: KeyLocation): Promise<string> {
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw 'Unsuported Method'
        }
        const keyPair: KeyPair = new KeyPair(KeyType.Ed25519);
        const publicKey: string = keyPair.public;
        if (this.vaults.has(did)) {
            this.vaults.get(did).set(keyLocation, keyPair);
        } else {
            let newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this.vaults.set(did, newVault);
        }
        return publicKey
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: string): Promise<string> {
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw 'Unsuported Method'
        }
        let secretKey: SecretKey = SecretKey.fromPrivateKey(privateKey);
        let publicKey: string = secretKey.publicKey();
        let keyPair: KeyPair = KeyPair.fromBase58(0, privateKey, publicKey);
        if (this.vaults.has(did)) {
            this.vaults.get(did).set(keyLocation, keyPair);
        } else {
            let newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this.vaults.set(did, newVault);
        }
        return publicKey
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        if (this.vaults.has(did)) {
            let vault: Map<KeyLocation, KeyPair> = this.vaults.get(did);
            if (vault.has(keyLocation)) {
                return true;
            }
        }
        return false
    }

    public async keyGet(did: DID, keyLocation: KeyLocation): Promise<string> {
        if (this.vaults.has(did)) {
            let vault: Map<KeyLocation, KeyPair> = this.vaults.get(did);
            if (vault.has(keyLocation)) {
                let keyPair: KeyPair = vault.get(keyLocation);
                return keyPair.public
            }
            throw 'Key location not found'
        }
        throw 'DID not found'
    }

    public async keyDel(did: DID, keyLocation: KeyLocation): Promise<void> {
        if (this.vaults.has(did)) {
            this.vaults.get(did).delete(keyLocation);
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        if (!this.vaults.has(did)) {
            throw 'DID not found'
        }
        let vault: Map<KeyLocation, KeyPair> = this.vaults.get(did);
        if (!vault.has(keyLocation)) {
            throw 'Key location not found'
        }
        let keyPair: KeyPair = vault.get(keyLocation);
        if (keyLocation.method !== MethodType.Ed25519VerificationKey2018()) {
            throw 'Unsuported Method'
        }
        let signature: Uint8Array = Ed25519.sign(data, keyPair.private);
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
    
    public async publishedGeneration(did: DID): Promise<Generation> {
        return this._publishedGenerations.get(did)
    }
    
    public async setPublishedGeneration(did: DID, generation: Generation): Promise<void> {
        this._publishedGenerations.set(did, generation);
    }
}

