// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IdentitySetup, AccountBuilder, MethodRelationship, } from './../../node/identity_wasm.js';


async function manipulateIdentity() {

    // ===========================================================================
    // Create Identity - Similar to create_did example
    // ===========================================================================

    // Create a new Account with the default configuration
    let builder = new AccountBuilder({});
    let account = await builder.createIdentity(new IdentitySetup());

    // ===========================================================================
    // Identity Manipulation
    // ===========================================================================

    // Add another Ed25519 verification method to the identity
    await account.createMethod({
        fragment: "my-next-key"
    })

    // Associate the newly created method with additional verification relationships
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
        serviceType: "MyCustomService",
        endpoint: "https://example.com"
    })

    // Remove the Ed25519 verification method
    await account.deleteMethod("my-next-key")

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did();

    // Print the DID of the created Identity.
    console.log(iotaDid.toString())

    // Print the local state of the DID Document
    console.log(account.document());
}

export { manipulateIdentity };
