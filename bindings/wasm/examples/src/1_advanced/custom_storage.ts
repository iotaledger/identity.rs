// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Ed25519, KeyPair, KeyStorage, KeyType } from "../../../node";

/** An insecure, in-memory `Storage` implementation that serves as an example.
This can be passed to the `AccountBuilder` to create accounts with this as the storage. */
// Refer to the `Storage` interface docs for high-level documentation of the individual methods.
export class MemStore implements KeyStorage {
    // The map from DIDs to state.
    private _keys: Map<string, KeyPair>;

    /** Creates a new, empty `MemStore` instance. */
    constructor() {
        this._keys = new Map();
    }

    public async generate(keyType: string): Promise<string> {
        let supportedKeyType;
        switch (keyType) {
            case "Ed25519":
                supportedKeyType = KeyType.Ed25519;
                break;
            case "X25519":
                supportedKeyType = KeyType.X25519;
                break;
            default:
                throw new Error(`unsupported key type ${keyType}`);
        }

        // Generate a new key pair with the given key type.
        const keyPair: KeyPair = new KeyPair(supportedKeyType);
        // TODO: Generate random string.
        const keyAlias: string = "very_random_key";

        this._keys.set(keyAlias, keyPair);

        return keyAlias;
    }

    public async public(privateKey: string): Promise<Uint8Array> {
        const keyPair = this._keys.get(privateKey);

        // Return the public key or an error if the vault or key does not exist.
        if (keyPair) {
            return keyPair.public();
        } else {
            // TODO: Return StorageError after porting it to Wasm.
            throw new Error("KeyPair not found");
        }
    }

    public async sign(privateKey: string, signing_algorithm: string, data: Uint8Array): Promise<Uint8Array> {
        const keyPair = this._keys.get(privateKey);

        if (keyPair) {
            // Use the `Ed25519` API to sign the given data with the private key.
            const signature: Uint8Array = Ed25519.sign(data, keyPair.private());
            return signature;
        } else {
            throw new Error("KeyPair not found");
        }
    }
}
