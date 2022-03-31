// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ChainState, DID, Document, Ed25519, KeyLocation, KeyPair, KeyType, Signature, Storage } from './../../node/identity_wasm.js';

// TODO: add thorough comments explaining what this is and how to use it with an Account.
export class MemStore implements Storage {
    // TODO: check if map key comparison works as-expected.
    //       I.e. does a parsed/deserialized DID map to the same DID object?
    private _chainStates: Map<DID, ChainState>;
    private _documents: Map<DID, Document>;
    private _vaults: Map<DID, Map<KeyLocation, KeyPair>>;
    private _list: Set<DID>;

    constructor() {
        this._chainStates = new Map();
        this._documents = new Map();
        this._vaults = new Map();
        this._list = new Set();
    }

    public async didCreate(network: string, fragment: string, privateKey: Uint8Array | undefined | null): Promise<[DID, KeyLocation]> {
        let keyPair;
        if (privateKey) {
            keyPair = KeyPair.tryFromPrivateKeyBytes(KeyType.Ed25519, privateKey);
        } else {
            keyPair = new KeyPair(KeyType.Ed25519);
        }


        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        let did: DID = new DID(keyPair.public(), network);

        if (this._list.has(did)) {
            throw new Error("identity already exists");
        } else {
            this._list.add(did);
        }

        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            vault.set(keyLocation, keyPair);
        } else {
            let newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this._vaults.set(did, newVault);
        }

        return [did, keyLocation];
    }

    public async didPurge(did: DID): Promise<boolean> {
        if (this._list.has(did)) {
            this._list.delete(did);
            this._chainStates.delete(did);
            this._documents.delete(did);
            this._vaults.delete(did);
            return true;
        }

        return false;
    }

    public async didExists(did: DID): Promise<boolean> {
        return this._list.has(did);
    }

    public async didList(): Promise<Array<DID>> {
        return Array.from(this._list);
    }

    public async keyGenerate(did: DID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        const keyPair: KeyPair = new KeyPair(keyType);
        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            vault.set(keyLocation, keyPair);
        } else {
            let newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this._vaults.set(did, newVault);
        }

        return keyLocation;
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: Uint8Array): Promise<void> {
        const keyPair: KeyPair = KeyPair.tryFromPrivateKeyBytes(keyLocation.keyType(), privateKey);

        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            vault.set(keyLocation, keyPair);
        } else {
            let newVault: Map<KeyLocation, KeyPair> = new Map([[keyLocation, keyPair]]);
            this._vaults.set(did, newVault);
        }
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            return vault.has(keyLocation);
        } else {
            return false
        }
    }

    public async keyPublic(did: DID, keyLocation: KeyLocation): Promise<Uint8Array> {
        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation);
            if (keyPair) {
                return keyPair.public()
            } else {
                throw new Error('Key location not found')
            }
        } else {
            throw new Error('DID not found')
        }
    }

    public async keyDelete(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            return vault.delete(keyLocation);
        } else {
            return false;
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        if (keyLocation.keyType() !== KeyType.Ed25519) {
            throw new Error('Unsupported Method')
        }

        const vault: Map<KeyLocation, KeyPair> | undefined = this._vaults.get(did);

        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation);

            if (keyPair) {
                let signature: Uint8Array = Ed25519.sign(data, keyPair.private());
                return new Signature(signature)
            } else {
                throw new Error('Key location not found')
            }
        } else {
            throw new Error('DID not found')
        }
    }

    public async chainStateGet(did: DID): Promise<ChainState | null | undefined> {
        return this._chainStates.get(did);
    }

    public async chainStateSet(did: DID, chainState: ChainState): Promise<void> {
        this._chainStates.set(did, chainState);
    }

    public async documentGet(did: DID): Promise<Document | null | undefined> {
        return this._documents.get(did)
    }

    public async documentSet(did: DID, document: Document): Promise<void> {
        this._documents.set(did, document);
    }

    public async flushChanges(): Promise<void> { }
}

