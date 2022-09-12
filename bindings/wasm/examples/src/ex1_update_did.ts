// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MethodRelationship, IotaDocument, IotaService, Timestamp, IotaIdentityClient } from '../../node';
import { IAliasOutput, IRent, TransactionHelper } from '@iota/iota.js';
import { API_ENDPOINT, createDid } from './util';
import { Client, MnemonicSecretManager } from '@cycraig/iota-client-wasm/node';
import { Bip39 } from '@iota/crypto.js';

/** Demonstrates how to update a DID document in an existing Alias Output. */
export async function updateIdentity() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        Mnemonic: Bip39.randomMnemonic()
    };

    // Creates a new wallet and identity (see "ex0_create_did" example).
    const { did } = await createDid(client, secretManager);

    // Resolve the latest state of the document.
    // Technically this is equivalent to the document above.
    const document: IotaDocument = await didClient.resolveDid(did);

    // Attach a new method relationship to the existing method.
    document.attachMethodRelationship(did.join("#key-1"), MethodRelationship.Authentication);

    // Add a new Service.
    const service: IotaService = new IotaService({
        id: did.join("#linked-domain"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org/"
    });
    document.insertService(service);
    document.setMetadataUpdated(Timestamp.nowUTC());

    // Resolve the latest output and update it with the given document.
    const aliasOutput: IAliasOutput = await didClient.updateDidOutput(document);

    // Because the size of the DID document increased, we have to increase the allocated storage deposit.
    // This increases the deposit amount to the new minimum.
    const rentStructure: IRent = await didClient.getRentStructure();
    aliasOutput.amount = TransactionHelper.getStorageDeposit(aliasOutput, rentStructure).toString();

    // Publish the output.
    const updated: IotaDocument = await didClient.publishDidOutput(secretManager, aliasOutput);
    console.log("Updated DID document:", JSON.stringify(updated, null, 2));
}
