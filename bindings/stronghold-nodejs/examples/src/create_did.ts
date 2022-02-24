// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountBuilder, ExplorerUrl } from "../../../wasm/node/identity_wasm.js";
import { Stronghold } from '../../dist'

/**
 * This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle
 * using the Account.
 */
async function createIdentity() {

    // Sets the location and password for the Stronghold
    //
    // Stronghold is an encrypted file that manages private keys.
    // It implements best practices for security and is the recommended way of handling private keys.
    let strongholdPath = "./example-strong.hodl";
    let password = "my-password";
    let stronghold = new Stronghold(strongholdPath, password, true);

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({
        storage: stronghold
    });
    let account = await builder.createIdentity();

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did().toString();

    // Print the DID of the created Identity.
    console.log(iotaDid)

    // Print the local state of the DID Document
    console.log(account.document());

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(iotaDid));

    // Add a new Ed25519 Verification Method to the identity.
    await account.createMethod({
        fragment: "key_1"
    })
}

export { createIdentity }