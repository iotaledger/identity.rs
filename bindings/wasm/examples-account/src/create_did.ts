// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountBuilder, ExplorerUrl } from './../../node/identity_wasm.js';

/**
 * This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle
 * using the Account.
 */
async function createIdentity() {

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder();
    let account = await builder.createIdentity();

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did().toString();

    // Print the DID of the created Identity.
    console.log(iotaDid)

    // Print the local state of the DID Document
    console.log(account.document());

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(iotaDid));
}

export { createIdentity };
