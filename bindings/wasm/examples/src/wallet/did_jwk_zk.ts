// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

import {
    ProofAlgorithm,
    JwkMemStore,
    KeyIdMemStore,
    Storage,
    CoreDocument
} from "@iota/identity-wasm/node";

/** Demonstrate how to create a ZK DID JWK Document */
export async function createDidJwkZk(){

    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    const document = await CoreDocument.newDidJwkZk(
        storage,
        ProofAlgorithm.BLS12381_SHA256,
        )

    console.log(JSON.stringify(document, null, 2));
}
