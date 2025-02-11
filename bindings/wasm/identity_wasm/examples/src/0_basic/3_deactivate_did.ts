// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDID } from "@iota/identity-wasm/node";
import { IotaClient } from "@iota/iota-sdk/client";
import {
    createDocumentForNetwork,
    getClientAndCreateAccount,
    getMemstorage,
    NETWORK_URL,
    TEST_GAS_BUDGET,
} from '../utils_alpha';

/** Demonstrates how to deactivate a DID in an Alias Output. */
export async function deactivateIdentity() {
    // create new clients and create new account
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();
    const storage = getMemstorage();
    const [unpublished, vmFragment1] = await createDocumentForNetwork(storage, network);
    const identityClient = await getClientAndCreateAccount(storage);

    // create new identity for this account and publish document for it
    const { output: identity } = await identityClient
        .createIdentity(unpublished)
        .finish()
        .execute(identityClient);
    const did = IotaDID.fromAliasId(identity.id(), identityClient.network());

    // Resolve the latest state of the document.
    // Technically this is equivalent to the document above.
    const resolved = await identityClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // Deactivate the DID by publishing an empty document.
    await identityClient.deactivateDidOutput(did, TEST_GAS_BUDGET);

    // Resolving a deactivated DID returns an empty DID document
    // with its `deactivated` metadata field set to `true`.
    let deactivated = await identityClient.resolveDid(did);
    console.log("Deactivated DID document:", JSON.stringify(deactivated, null, 2));
    if (deactivated.metadataDeactivated() !== true) {
        throw new Error("Failed to deactivate DID document");
    }

    // Re-activate the DID by publishing a valid DID document.
    console.log("Publishing this:", JSON.stringify(resolved, null, 2));
    await identityClient
      .publishDidDocumentUpdate(resolved, TEST_GAS_BUDGET);

    // Resolve the reactivated DID document.
    let resolvedReactivated = await identityClient.resolveDid(did);
    console.log("Reactivated DID document:", JSON.stringify(resolvedReactivated, null, 2));
    if (resolvedReactivated.metadataDeactivated() === true) {
        throw new Error("Failed to reactivate DID document");
    }
}
