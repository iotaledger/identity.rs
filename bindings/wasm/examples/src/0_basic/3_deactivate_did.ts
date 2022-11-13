// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bip39 } from "@iota/crypto.js";
import { Client, MnemonicSecretManager } from "@iota/iota-client-wasm/node";
import { IAliasOutput, IRent, TransactionHelper } from "@iota/iota.js";
import { IotaDocument, IotaIdentityClient } from "../../../node";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how to deactivate a DID in an Alias Output. */
export async function deactivateIdentity() {
    const client = await Client.new({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    let { document } = await createDid(client, secretManager);
    const did = document.id();

    // Resolve the latest state of the DID document, so we can reactivate it later.
    // Technically this is equivalent to the document above.
    document = await didClient.resolveDid(did);

    // Deactivate the DID by publishing an empty document.
    // This process can be reversed since the Alias Output is not destroyed.
    // Deactivation may only be performed by the state controller of the Alias Output.
    let deactivatedOutput: IAliasOutput = await didClient.deactivateDidOutput(did);

    // Optional: reduce and reclaim the storage deposit, sending the tokens to the state controller.
    const rentStructure: IRent = await didClient.getRentStructure();
    deactivatedOutput.amount = TransactionHelper.getStorageDeposit(deactivatedOutput, rentStructure).toString();

    // Publish the deactivated DID document.
    await didClient.publishDidOutput(secretManager, deactivatedOutput);

    // Resolving a deactivated DID returns an empty DID document
    // with its `deactivated` metadata field set to `true`.
    let deactivated: IotaDocument = await didClient.resolveDid(did);
    console.log("Deactivated DID document:", JSON.stringify(deactivated, null, 2));
    if (deactivated.metadataDeactivated() !== true) {
        throw new Error("Failed to deactivate DID document");
    }

    // Re-activate the DID by publishing a valid DID document.
    let reactivatedOutput: IAliasOutput = await didClient.updateDidOutput(document);

    // Increase the storage deposit to the minimum again, if it was reclaimed during deactivation.
    reactivatedOutput.amount = TransactionHelper.getStorageDeposit(reactivatedOutput, rentStructure).toString();
    await didClient.publishDidOutput(secretManager, reactivatedOutput);

    // Resolve the reactivated DID document.
    let reactivated: IotaDocument = await didClient.resolveDid(did);
    if (reactivated.metadataDeactivated() === true) {
        throw new Error("Failed to reactivate DID document");
    }
}
