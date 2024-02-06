// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaIdentityClient, JwkMemStore, KeyIdMemStore, Storage } from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. */
export async function deleteIdentity() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    // const { address, document } = await createDid(client, secretManager);
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { address, document } = await createDid(
        client,
        secretManager,
        storage,
    );
    const did = document.id();

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
