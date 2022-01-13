// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {IdentitySetup, AccountBuilder, } from './../../node/identity_wasm.js';

/**
 * This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle
 * using the Account.
 */
async function createIdentity() {

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({});
    let account = await builder.createIdentity(new IdentitySetup());

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did();

    // Print the DID of the created Identity.
    console.log(iotaDid.toString())

    // Print the local state of the DID Document
    console.log(account.document());
}

export { createIdentity };
