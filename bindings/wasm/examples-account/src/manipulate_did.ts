// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {AccountBuilder, ExplorerUrl, MethodContent, MethodRelationship, Storage} from './../../node';

/**
 * This example demonstrates how to manipulate a DID Document by adding/removing
 * Verification Methods and Services.
 */
async function manipulateIdentity(storage?: Storage) {

    // ===========================================================================
    // Create Identity - Similar to create_did example
    // ===========================================================================

    // Create a new Account with the default configuration.
    let builder = new AccountBuilder({
        storage,
    });
    let account = await builder.createIdentity();

    // ===========================================================================
    // Identity Manipulation
    // ===========================================================================

    // Add another Ed25519 verification method to the identity.
    await account.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "my-next-key"
    })

    // Associate the newly created method with additional verification relationships.
    await account.attachMethodRelationships({
        fragment: "my-next-key",
        relationships: [
            MethodRelationship.CapabilityDelegation,
            MethodRelationship.CapabilityInvocation
        ]
    })

    // Add a new service to the identity.
    await account.createService({
        fragment: "my-service-1",
        type: "MyCustomService",
        endpoint: "https://example.com"
    })

    // Remove the Ed25519 verification method.
    await account.deleteMethod({fragment: "my-next-key"})

    // Retrieve the DID of the newly created identity.
    let did = account.did();

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));
}

export {manipulateIdentity};
