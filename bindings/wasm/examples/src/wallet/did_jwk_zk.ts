// SPDX-License-Identifier: Apache-2.0

import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    MethodScope,
    Storage,
    CoreDocument
} from "@iota/identity-wasm/node";
import { AliasOutput, Client, MnemonicSecretManager, SecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function createDidJwkZk(){

    const mnemonicSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    const document = await CoreDocument.newDidJwk(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,)

    console.log(JSON.stringify(document, null, 2));
}
