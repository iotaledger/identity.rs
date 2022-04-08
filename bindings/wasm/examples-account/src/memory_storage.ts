// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ChainState, DID, Document, Ed25519, KeyLocation, KeyPair, KeyType, Signature, Storage, StorageTestSuite } from './../../node/identity_wasm.js';

// TODO: add thorough comments explaining what this is and how to use it with an Account.
export class MemStore implements Storage {
    private _chainStates: Map<string, ChainState>;
    private _documents: Map<string, Document>;
    private _vaults: Map<string, Map<string, KeyPair>>;

    constructor() {
        this._chainStates = new Map();
        this._documents = new Map();
        this._vaults = new Map();
    }

    public async didCreate(network: string, fragment: string, privateKey?: Uint8Array): Promise<[DID, KeyLocation]> {
        let keyPair;
        if (privateKey) {
            keyPair = KeyPair.tryFromPrivateKeyBytes(KeyType.Ed25519, privateKey);
        } else {
            keyPair = new KeyPair(KeyType.Ed25519);
        }

        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        const did: DID = new DID(keyPair.public(), network);

        if (this._vaults.has(did.toString())) {
            throw new Error("identity already exists");
        }

        const vault = this._vaults.get(did.toString());

        if (vault) {
            vault.set(keyLocation.toString(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.toString(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }

        return [did, keyLocation];
    }

    public async didPurge(did: DID): Promise<boolean> {
        if (this._vaults.has(did.toString())) {
            this._chainStates.delete(did.toString());
            this._documents.delete(did.toString());
            this._vaults.delete(did.toString());
            return true;
        }

        return false;
    }

    public async didExists(did: DID): Promise<boolean> {
        return this._vaults.has(did.toString());
    }

    public async didList(): Promise<Array<DID>> {
        return Array.from(this._vaults.keys()).map((did) => DID.parse(did));
    }

    public async keyGenerate(did: DID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        const keyPair: KeyPair = new KeyPair(keyType);
        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        const vault = this._vaults.get(did.toString());

        if (vault) {
            vault.set(keyLocation.toString(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.toString(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }

        return keyLocation;
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: Uint8Array): Promise<void> {
        const keyPair: KeyPair = KeyPair.tryFromPrivateKeyBytes(keyLocation.keyType(), privateKey);

        const vault = this._vaults.get(did.toString());

        if (vault) {
            vault.set(keyLocation.toString(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.toString(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        const vault = this._vaults.get(did.toString());

        if (vault) {
            return vault.has(keyLocation.toString());
        } else {
            return false
        }
    }

    public async keyPublic(did: DID, keyLocation: KeyLocation): Promise<Uint8Array> {
        const vault = this._vaults.get(did.toString());


        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation.toString());

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
        const vault = this._vaults.get(did.toString());

        if (vault) {
            return vault.delete(keyLocation.toString());
        } else {
            return false;
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        if (keyLocation.keyType() !== KeyType.Ed25519) {
            throw new Error('Unsupported Method')
        }

        const vault = this._vaults.get(did.toString());

        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation.toString());

            if (keyPair) {
                const signature: Uint8Array = Ed25519.sign(data, keyPair.private());
                return new Signature(signature)
            } else {
                throw new Error('Key location not found')
            }
        } else {
            throw new Error('DID not found')
        }
    }

    public async chainStateGet(did: DID): Promise<ChainState | null | undefined> {
        return this._chainStates.get(did.toString());
    }

    public async chainStateSet(did: DID, chainState: ChainState): Promise<void> {
        this._chainStates.set(did.toString(), chainState);
    }

    public async documentGet(did: DID): Promise<Document | null | undefined> {
        return this._documents.get(did.toString())
    }

    public async documentSet(did: DID, document: Document): Promise<void> {
        this._documents.set(did.toString(), document);
    }

    public async flushChanges(): Promise<void> { }
}

export async function storageTestSuite() {
    await StorageTestSuite.didCreateGenerateKeyTest(new MemStore());
    await StorageTestSuite.didCreatePrivateKeyTest(new MemStore());
    await StorageTestSuite.keyGenerateTest(new MemStore());
    await StorageTestSuite.keyDeleteTest(new MemStore());
    await StorageTestSuite.keyInsertTest(new MemStore());
    await StorageTestSuite.didListTest(new MemStore());
    await StorageTestSuite.keySignEd25519Test(new MemStore());
    await StorageTestSuite.didPurgeTest(new MemStore());
}
