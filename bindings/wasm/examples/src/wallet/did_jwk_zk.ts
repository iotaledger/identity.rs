// SPDX-License-Identifier: Apache-2.0

import {
    IotaDID,
    IotaDocument,
    ProofAlgorithm,
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

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    const document = await CoreDocument.newDidJwkZk(
        storage,
        ProofAlgorithm.BLS12381_SHA256,
        )

    console.log(JSON.stringify(document, null, 2));
}
