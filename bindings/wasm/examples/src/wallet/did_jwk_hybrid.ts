// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

import {
    CompositeAlgId,
    JwkMemStore,
    KeyIdMemStore,
    Storage,
    CoreDocument
} from "@iota/identity-wasm/node";

/** Demonstrate how to create a DID CompositeJWK Document */
export async function createDidJwkHybrid(){

    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    const document = await CoreDocument.newDidCompositeJwk(
        storage,
        CompositeAlgId.IdMldsa44Ed25519Sha512)

    console.log(JSON.stringify(document, null, 2));
}
