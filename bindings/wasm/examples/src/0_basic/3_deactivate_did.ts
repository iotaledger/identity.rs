// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDocument, IotaIdentityClient, JwkMemStore, KeyIdMemStore, Storage } from "@iota/identity-wasm/node";
import { AliasOutput, Client, IRent, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/** Demonstrates how to deactivate a DID in an Alias Output. */
export async function deactivateIdentity() {
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
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    const did = document.id();

    // Resolve the latest state of the DID document, so we can reactivate it later.
    // Technically this is equivalent to the document above.
    document = await didClient.resolveDid(did);

    // Deactivate the DID by publishing an empty document.
    // This process can be reversed since the Alias Output is not destroyed.
    // Deactivation may only be performed by the state controller of the Alias Output.
    let deactivatedOutput: AliasOutput = await didClient.deactivateDidOutput(did);

    // Optional: reduce and reclaim the storage deposit, sending the tokens to the state controller.
    const rentStructure: IRent = await didClient.getRentStructure();

    deactivatedOutput = await client.buildAliasOutput({
        ...deactivatedOutput,
        amount: Utils.computeStorageDeposit(deactivatedOutput, rentStructure),
        aliasId: deactivatedOutput.getAliasId(),
        unlockConditions: deactivatedOutput.getUnlockConditions(),
    });

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
    let reactivatedOutput: AliasOutput = await didClient.updateDidOutput(document);

    // Increase the storage deposit to the minimum again, if it was reclaimed during deactivation.
    reactivatedOutput = await client.buildAliasOutput({
        ...reactivatedOutput,
        amount: Utils.computeStorageDeposit(reactivatedOutput, rentStructure),
        aliasId: reactivatedOutput.getAliasId(),
        unlockConditions: reactivatedOutput.getUnlockConditions(),
    });

    await didClient.publishDidOutput(secretManager, reactivatedOutput);

    // Resolve the reactivated DID document.
    let reactivated: IotaDocument = await didClient.resolveDid(did);
    if (reactivated.metadataDeactivated() === true) {
        throw new Error("Failed to reactivate DID document");
    }
}
