// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {AccountBuilder, ExplorerUrl, MethodContent, Storage} from '@iota/identity-wasm/node';

/**
 * This example demonstrates how to create multiple identities from a builder
 * and how to load existing identities into an account.
 */
async function multipleIdentities(storage?: Storage) {

    // Create an AccountBuilder to make it easier to create multiple identities.
    // Every account created from the builder will use the same storage.
    let builder = new AccountBuilder({
        storage,
    });

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let account1 = await builder.createIdentity();

    // Create a second identity.
    let account2 = await builder.createIdentity();

    // Retrieve the did of the identity that account1 manages.
    let did1 = account1.did();

    // Suppose we're done with account1 and free it.
    account1.free();

    // Now we want to modify the first identity - how do we do that?
    // We can load the identity from storage into an account using the builder.
    let account1Reconstructed = await builder.loadIdentity(did1);

    // Now we can make modifications to the identity.
    // We can even do so concurrently.
    const account1Promise = account1Reconstructed.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "my_key"
    })
    const account2Promise = account2.createMethod({
        content: MethodContent.GenerateX25519(),
        fragment: "my_other_key"
    })

    await Promise.all([account1Promise, account2Promise]);

    // Print the Explorer URL for the DID.
    let did = account1Reconstructed.did().toString();
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));
}

export { multipleIdentities };
