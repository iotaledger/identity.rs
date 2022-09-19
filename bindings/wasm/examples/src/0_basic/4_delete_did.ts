// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager } from "@iota/iota-client-wasm/node";
import { IotaIdentityClient } from "../../../node";
import { API_ENDPOINT, createDid } from "../util";
import { Bip39 } from "@iota/crypto.js";

/** Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. */
export async function deleteIdentity() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        Mnemonic: Bip39.randomMnemonic()
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    const { address, did } = await createDid(client, secretManager);

    // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
    // This operation is *not* reversible.
    // Deletion can only be done by the governor of the Alias Output.
    const destinationAddress = address;
    await didClient.deleteDidOutput(secretManager, destinationAddress, did);

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
