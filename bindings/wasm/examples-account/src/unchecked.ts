// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {AccountBuilder, Timestamp, ExplorerUrl } from './../../node/identity_wasm.js';

/**
 * Updates an Identity without validation and publishes it to the tangle.
 */
async function unchecked() {

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder();
    let account = await builder.createIdentity();

    // Retrieve the did of the newly created identity.
    let iotaDid = account.did().toString();

    // Print the DID of the created Identity.
    console.log(`did: ${iotaDid}`);

    // Get a copy of the document this account manages.
    // We will apply updates to the document, and overwrite the account's current document.
    let document = account.document();

    // Print the local state of the DID Document
    console.log(`Document before update`, document);

    // Override the updated field timestamp to 01.01.1990 00:00:00.
    // because we can. This is usually set automatically when updating via the `Account`.
    document.metadataUpdated = Timestamp.parse("1900-01-01T00:00:00Z")

    // Update the identity without validation and publish the result to the Tangle
    // (depending on the account's autopublish setting).
    // The responsibility is on the caller to provide a valid document which the account
    // can continue to use. Failing to do so can corrupt the identity; use with caution!
    await account.updateDocumentUnchecked(document);

    // Print the local state of the DID Document after the update.
    console.log(`Document after update`, account.document());

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(iotaDid));
     
}

export { unchecked }
