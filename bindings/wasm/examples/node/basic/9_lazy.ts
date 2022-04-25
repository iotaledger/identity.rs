// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    ExplorerUrl,
    Storage
} from '@iota/identity-wasm/node';

/**
 * This example demonstrates how to take control over publishing DID updates manually,
 * instead of the default automated behavior.
 */
async function lazy(storage?: Storage) {

    // Create a new Account with auto publishing set to false.
    // This means updates are not pushed to the tangle automatically.
    // Rather, when we publish, multiple updates are batched together.
    let builder = new AccountBuilder({
        autopublish: false,
        storage,
    });
    let account = await builder.createIdentity();

    // Add a new service to the local DID document.
    await account.createService({
        fragment: "example-service",
        type: "LinkedDomains",
        endpoint: "https://example.org"
    })

    // Publish the newly created DID document,
    // including the new service, to the tangle.
    await account.publish();

    // Add another service.
    await account.createService({
        fragment: "another-service",
        type: "LinkedDomains",
        endpoint: "https://example.org"
    });

    // Delete the previously added service.
    await account.deleteService({
        fragment: "example-service"
    });

    // Publish the updates as one message to the tangle.
    await account.publish();

    // Retrieve the DID of the newly created identity.
    let did = account.did();

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));
}

export { lazy };
