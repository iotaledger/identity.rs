// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

import {
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    Storage,
    CoreDocument
} from "@iota/identity-wasm/node";

/** Demonstrate how to create a traditional DID JWK Document */
export async function createDidJwk(){

    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    const document: CoreDocument = await CoreDocument.newDidJwk(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,)

    console.log(JSON.stringify(document, null, 2));
    console.log("fragment" + document.fragmentJwk());
}
