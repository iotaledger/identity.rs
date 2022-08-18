// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {createIdentity} from "./ex0_create_did";
import {Bech32Helper} from "@iota/iota.js";

/** Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. */
export async function deleteIdentity() {
    // Creates a new wallet and identity (see "ex0_create_did" example).
    const {didClient, secretManager, walletAddressBech32, did} = await createIdentity();

    // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
    // This operation is *not* reversible.
    // Deletion can only be done by the governor of the Alias Output.
    const destinationAddress = Bech32Helper.addressFromBech32(walletAddressBech32, await didClient.getNetworkHrp());
    await didClient.deleteDidOutput(secretManager, destinationAddress, did);

    // Wait for the node to index the new state.
    await new Promise(f => setTimeout(f, 5000));

    // Attempting to resolve a deleted DID results in a `NotFound` error.
    let deleted = false;
    try {
        await didClient.resolveDid(did);
    } catch (err) {
        deleted = true;
    }
    if (!deleted) {
        throw new Error("failed to delete DID");
    }
}
