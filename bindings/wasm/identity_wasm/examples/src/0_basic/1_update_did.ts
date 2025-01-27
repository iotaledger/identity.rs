// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    JwkMemStore,
    JwsAlgorithm,
    MethodRelationship,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod,
} from "@iota/identity-wasm/node";

import {
    createDidDocument,
    getClientAndCreateAccount,
    getMemstorage,
    TEST_GAS_BUDGET,
} from "../utils_alpha";

/** Demonstrates how to update a DID document in an existing Alias Output. */
export async function updateIdentity() {
    // create new client to interact with chain and get funded account with keys
    const storage = getMemstorage();
    const identityClient = await getClientAndCreateAccount(storage);
  
    // create new DID document and publish it
    let [document, vmFragment1] = await createDidDocument(identityClient, storage);
    let did = document.id();

    // Resolve the latest state of the document.
    // Technically this is equivalent to the document above.
    document = await identityClient.resolveDid(did);

    // Insert a new Ed25519 verification method in the DID document.
    await document.generateMethod(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,
        "#key-2",
        MethodScope.VerificationMethod(),
    );

    // Attach a new method relationship to the inserted method.
    document.attachMethodRelationship(did.join("#key-2"), MethodRelationship.Authentication);


    // Add a new Service.
    const service: Service = new Service({
        id: did.join("#linked-domain"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org/",
    });
    document.insertService(service);
    document.setMetadataUpdated(Timestamp.nowUTC());

    // Remove a verification method.
    let originalMethod = document.resolveMethod(vmFragment1) as VerificationMethod;
    await document.purgeMethod(storage, originalMethod?.id());

    let updated = identityClient
        .publishDidDocumentUpdate(document.clone(), TEST_GAS_BUDGET);
    console.log(`Updated DID document result: ${JSON.stringify(updated, null, 2)}`);
}
