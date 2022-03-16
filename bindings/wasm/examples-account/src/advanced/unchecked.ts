// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountBuilder, Timestamp, ExplorerUrl, Storage } from '../../../node/identity_wasm.js';

/**
 * This example demonstrates how to update the custom properties of a DID document directly
 * and publish it without performing validation.
 */
async function unchecked(storage?: Storage) {

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    const builder = new AccountBuilder({
        storage,
    });
    const account = await builder.createIdentity();

    // Get a copy of the document this account manages.
    // We will apply updates to the document, and overwrite the account's current document.
    const document = account.document();

    // Print the local state of the DID Document
    console.log(`[Example] Document before update`, document);

    // Add a custom property to the document.
    document.setPropertyUnchecked("myCustomPropertyKey", "value");

    // Override the updated field timestamp to 01.01.1900 00:00:00.
    // because we can. This is usually set automatically when updating via the `Account`.
    document.setMetadataUpdated(Timestamp.parse("1900-01-01T00:00:00Z"));

    // Update the identity without validation and publish the result to the Tangle
    // (depending on the account's autopublish setting).
    // The responsibility is on the caller to provide a valid document which the account
    // can continue to use. Failing to do so can corrupt the identity; use with caution!
    await account.updateDocumentUnchecked(document);

    // Print the local state of the DID Document after the update.
    console.log(`[Example] Document after update`, account.document());

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(account.did()));

}

export { unchecked }
