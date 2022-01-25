// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Ed25519, Signature, KeyPair, SecretKey} from '@iota/identity-wasm';

class MemStore {
    constructor() {
        this.expand = false;
        this.publishedGenerations = new Map();
        this.didLeases = new Map();
        this.chainStates = new Map();
        this.states = new Map();
        this.vaults = new Map();
    }
    
    get expand() {
        return this.expand
    }
    
    set expand(value) {
        this.expand = value
    }
    
    get vaults() {
        return this.vaults
    }

    setPassword(_encryptionKey) {
        return new Promise((resolve, _reject) => {
            resolve();
        });
    }

    flushChanges() {
        return new Promise((resolve, _reject) => {
            resolve();
        });
    }

    leaseDid(did) {
        return new Promise((resolve, reject) => {
            if (this.didLeases.has(did)) {
                let didLease = this.didLeases.get(did);
                if (didLease.load()) {
                    return reject('Identity in Use');
                } 
                didLease.store(true);
                return resolve(didLease);
            
            }
            let didLease = new didLease();
            this.didLeases.set(did, didLease);
            resolve(didLease);
        });
    }

    keyNew(did, keyLocation) {
        return new Promise((resolve, _reject) => {
            if (keyLocation.method() === 0) {
                const key = new KeyPair(KeyType.Ed25519);
                const publicKey = key.public();
                if (this.vaults.has(did)) {
                    this.vaults.get(did).set(keyLocation, publicKey);
                } else {
                    let newVault = new Map([keyLocation, publicKey]);
                    this.vaults.set(did, newVault);
                }
                resolve(publicKey);
            }
        });
    }

    keyInsert(did, keyLocation, privateKey) {
        return new Promise((resolve, _reject) => {
            if (keyLocation.method() === 0) {
                let secretKey = SecretKey.fromPrivateKey(privateKey);
                let publicKey = secretKey.publicKey();
                let keyPair = KeyPair.fromBase58(0, privateKey, publicKey);
                if (this.vaults.has(did)) {
                    this.vaults.get(did).set(keyLocation, keyPair);
                } else {
                    let newVault = new Map([keyLocation, keyPair]);
                    this.vaults.set(did, newVault);
                }
                resolve(publicKey);
            }
        });
    }

    keyExists(did, keyLocation) {
        return new Promise((resolve, _reject) => {
            if (this.vaults.has(did)) {
                let vault = this.vaults.get(did);
                if (vault.has(keyLocation)) {
                    return resolve(true);
                }
            }
            resolve(false);
        });
    }

    keyGet(did, keyLocation) {
        return new Promise((resolve, reject) => {
            if (this.vaults.has(did)) {
                let vault = this.vaults.get(did);
                if (vault.has(keyLocation)) {
                    let keyPair = vault.get(keyLocation);
                    return resolve(keyPair.public());
                }
                return reject('Key location not found');
            }
            reject('DID not found');
        });
    }

    keyDel(did, keyLocation) {
        return new Promise((resolve, _reject) => {
            if (this.vaults.has(did)) {
                this.vaults.get(did).delete(keyLocation);
            }
            resolve();
        });
    }

    keySign(did, keyLocation, data) {
        return new Promise((resolve, reject) => {
            if (!this.vaults.has(did)) {
                return reject('DID not found')
            }
            let vault = this.vaults.get(did);
            if (!vault.has(keyLocation)) {
                return reject('Key location not found')
            }
            let keyPair = vault.get(keyLocation);
            if (keyLocation.method() !== 0) {
                return reject('Unsuported Method')
            }
            let signature = Ed25519.sign(data, keyPair.private());
            resolve(new Signature(keyPair.public(), signature))
        });
    }

    chainState(did) {
        return new Promise((resolve, _reject) => {
            resolve(this.chainStates.get(did));
        });
    }

    setChainState(did, chainState) {
        return new Promise((resolve, _reject) => {
            this.chainStates.set(did, chainState);
            resolve();
        });
    }

    state(did) {
        return new Promise((resolve, _reject) => {
            resolve(this.states.get(did));
        });
    }

    setState(did, identityState) {
        return new Promise((resolve, _reject) => {
            this.chainStates.set(did, identityState);
            resolve();
        });
    }

    purge(did) {
        return new Promise((resolve, _reject) => {
            this.chainStates.delete(did);
            this.states.delete(did);
            this.vaults.delete(did);
            resolve();
        });
    }

    publishedGeneration(did) {
        return new Promise((resolve, _reject) => {
            resolve(this.publishedGenerations.get(did));
        });
    }

    setPublishedGeneration(did, generation) {
        return new Promise((resolve, _reject) => {
            this.chainStates.set(did, generation);
            resolve();
        });
    }
}