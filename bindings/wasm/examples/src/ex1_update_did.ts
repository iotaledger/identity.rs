// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { MethodRelationship, IotaDocument, IotaService, Timestamp, IotaVerificationMethod, KeyPair, KeyType, MethodScope } from '../../node';
import { IAliasOutput, IRent, TransactionHelper } from '@iota/iota.js';

import { createIdentity } from "./ex0_create_did";

/** Demonstrates how to update a DID document in an existing Alias Output. */
export async function updateIdentity() {
    // Creates a new wallet and identity (see "ex0_create_did" example).
    const { didClient, secretManager, did } = await createIdentity();

    // Resolve the latest state of the document.
    // Technically this is equivalent to the document above.
    const document: IotaDocument = await didClient.resolveDid(did);

    // Insert a new Ed25519 verification method in the DID document.
    let keypair = new KeyPair(KeyType.Ed25519);
    let method = new IotaVerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-2");
    document.insertMethod(method, MethodScope.VerificationMethod());

    // Attach a new method relationship to the inserted method.
    document.attachMethodRelationship(did.join("#key-2"), MethodRelationship.Authentication);

    // Add a new Service.
    const service: IotaService = new IotaService({
        id: did.join("#linked-domain"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org/"
    });
    document.insertService(service);
    document.setMetadataUpdated(Timestamp.nowUTC());

    // Remove a verification method.
    let originalMethod = document.resolveMethod("key-1") as IotaVerificationMethod;
    document.removeMethod(originalMethod?.id());

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
