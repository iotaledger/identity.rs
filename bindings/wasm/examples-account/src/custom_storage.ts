// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { DID, Ed25519, KeyLocation, KeyPair, KeyType, Signature, Storage, StorageTestSuite, EncryptionAlgorithm, CekAlgorithm, EncryptedData } from '../../node/identity_wasm.js';

/** An insecure, in-memory `Storage` implementation that serves as an example.
This can be passed to the `AccountBuilder` to create accounts with this as the storage. */
// Refer to the `Storage` interface docs for high-level documentation of the individual methods.
export class MemStore implements Storage {
    // We use strings as keys rather than DIDs or KeyLocations because Maps use
    // referential equality for object keys, and thus a primitive type needs to be used instead.

    // The map from DIDs to state.
    private _blobs: Map<string, Uint8Array>;
    // Map of DID state blobs.
    private _vaults: Map<string, Map<string, KeyPair>>;

    /** Creates a new, empty `MemStore` instance. */
    constructor() {
        this._blobs = new Map();
        this._vaults = new Map();
    }

    public async didCreate(network: string, fragment: string, privateKey?: Uint8Array): Promise<[DID, KeyLocation]> {
        // Extract a `KeyPair` from the passed private key or generate a new one.
        // For `did_create` we can assume the `KeyType` to be `Ed25519` because
        // that is the only currently available signature type.
        let keyPair;
        if (privateKey) {
            keyPair = KeyPair.tryFromPrivateKeyBytes(KeyType.Ed25519, privateKey);
        } else {
            keyPair = new KeyPair(KeyType.Ed25519);
        }

        // We create the location at which the key pair will be stored.
        // Most notably, this uses the public key as an input.
        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        // Next we use the public key to derive the initial DID.
        const did: DID = new DID(keyPair.public(), network);

        // We use the vaults as the index of DIDs stored in this storage instance.
        // If the DID already exists, we need to return an error. We don't want to overwrite an existing DID.
        if (this._vaults.has(did.toString())) {
            throw new Error("identity already exists");
        }

        const vault = this._vaults.get(did.toString());

        // Get the existing vault and insert the key pair,
        // or insert a new vault with the key pair.
        if (vault) {
            vault.set(keyLocation.canonical(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.canonical(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }

        return [did, keyLocation];
    }

    public async didPurge(did: DID): Promise<boolean> {
        // This method is supposed to be idempotent,
        // so we only need to do work if the DID still exists.
        // The return value signals whether the DID was actually removed during this operation.
        if (this._vaults.has(did.toString())) {
            this._blobs.delete(did.toString());
            this._vaults.delete(did.toString());
            return true;
        }

        return false;
    }

    public async didExists(did: DID): Promise<boolean> {
        return this._vaults.has(did.toString());
    }

    public async didList(): Promise<Array<DID>> {
        // Get all keys from the vaults and parse them into DIDs.
        return Array.from(this._vaults.keys()).map((did) => DID.parse(did));
    }

    public async keyGenerate(did: DID, keyType: KeyType, fragment: string): Promise<KeyLocation> {
        // Generate a new key pair with the given key type.
        const keyPair: KeyPair = new KeyPair(keyType);
        // Derive the key location from the fragment and public key and set the `KeyType` of the location.
        const keyLocation: KeyLocation = new KeyLocation(KeyType.Ed25519, fragment, keyPair.public());

        const vault = this._vaults.get(did.toString());

        // Get the existing vault and insert the key pair,
        // or insert a new vault with the key pair.
        if (vault) {
            vault.set(keyLocation.canonical(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.canonical(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }

        // Return the location at which the key was generated.
        return keyLocation;
    }

    public async keyInsert(did: DID, keyLocation: KeyLocation, privateKey: Uint8Array): Promise<void> {
        // Reconstruct the key pair from the given private key with the location's key type.
        const keyPair: KeyPair = KeyPair.tryFromPrivateKeyBytes(keyLocation.keyType(), privateKey);

        // Get the vault for the given DID.
        const vault = this._vaults.get(did.toString());

        // Get the existing vault and insert the key pair,
        // or insert a new vault with the key pair.
        if (vault) {
            vault.set(keyLocation.canonical(), keyPair);
        } else {
            const newVault = new Map([[keyLocation.canonical(), keyPair]]);
            this._vaults.set(did.toString(), newVault);
        }
    }

    public async keyExists(did: DID, keyLocation: KeyLocation): Promise<boolean> {
        // Get the vault for the given DID.
        const vault = this._vaults.get(did.toString());

        // Within the DID vault, check for existence of the given location.
        if (vault) {
            return vault.has(keyLocation.canonical());
        } else {
            return false
        }
    }

    public async keyPublic(did: DID, keyLocation: KeyLocation): Promise<Uint8Array> {
        // Get the vault for the given DID.
        const vault = this._vaults.get(did.toString());

        // Return the public key or an error if the vault or key does not exist.
        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation.canonical());
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
        // Get the vault for the given DID.
        const vault = this._vaults.get(did.toString());

        // This method is supposed to be idempotent, so we delete the key
        // if it exists and return whether it was actually deleted during this operation.
        if (vault) {
            return vault.delete(keyLocation.canonical());
        } else {
            return false;
        }
    }

    public async keySign(did: DID, keyLocation: KeyLocation, data: Uint8Array): Promise<Signature> {
        if (keyLocation.keyType() !== KeyType.Ed25519) {
            throw new Error('Unsupported Method')
        }

        // Get the vault for the given DID.
        const vault = this._vaults.get(did.toString());

        if (vault) {
            const keyPair: KeyPair | undefined = vault.get(keyLocation.canonical());

            if (keyPair) {
                // Use the `Ed25519` API to sign the given data with the private key.
                const signature: Uint8Array = Ed25519.sign(data, keyPair.private());
                // Construct a new `Signature` wrapper with the returned signature bytes.
                return new Signature(signature)
            } else {
                throw new Error('Key location not found')
            }
        } else {
            throw new Error('DID not found')
        }
    }

    public async dataEncrypt(did: DID, plaintext: Uint8Array, associatedData: Uint8Array, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, publicKey: Uint8Array): Promise<EncryptedData> {
        throw new Error('not yet implemented')
    }

    public async dataDecrypt(did: DID, data: EncryptedData, encryptionAlgorithm: EncryptionAlgorithm, cekAlgorithm: CekAlgorithm, privateKey: KeyLocation): Promise<Uint8Array> {
        throw new Error('not yet implemented')
    }

    public async blobGet(did: DID): Promise<Uint8Array | undefined> {
        // Lookup the state of the given DID.
        return this._blobs.get(did.toString());
    }

    public async blobSet(did: DID, value: Uint8Array): Promise<void> {
        // Set the state of the given DID.
        this._blobs.set(did.toString(), value);
    }

    public async flushChanges(): Promise<void> {
        // The MemStore doesn't need to flush changes to disk or any other persistent store,
        // which is why this function does nothing.
    }
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
