// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IdentitySetup, AccountBuilder } from './../../node/identity_wasm.js';

async function lazy() {

    // Create a new Account with auto publishing set to false.
    // This means updates are not pushed to the tangle automatically.
    // Rather, when we publish, multiple updates are batched together.
    let builder = new AccountBuilder({
        autopublish: false
    });
    let account = await builder.createIdentity(new IdentitySetup());

    // Add a new service to the local DID document.
    await account.createService({
        fragment: "example-service",
        serviceType: "LinkedDomains",
        endpoint: "https://example.org"
    })

    // Publish the newly created DID document,
    // including the new service, to the tangle.
    await account.publish();

    // Add another service.
    await account.createService({
        fragment: "another-service",
        serviceType: "LinkedDomains",
        endpoint: "https://example.org"
    });

    // Delete the previously added service.
    await account.deleteService("example-service");

    // Publish the updates as one message to the tangle.
    await account.publish();

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did();

    // Print the DID of the created Identity.
    console.log(iotaDid.toString())

    // Print the local state of the DID Document
    console.log(account.document());
}

export { lazy };
