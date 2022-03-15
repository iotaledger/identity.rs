// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    KeyPair,
    KeyType,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod
} from '@iota/identity-wasm';
import {createIdentity} from "./create_did";

/**
 This example shows how to add more to an existing DID Document.
 The two main things to add are Verification Methods and Services.
 A verification method adds public keys, which can be used to digitally sign things as an identity.
 The services provide metadata around the identity via URIs. These can be URLs, but can also emails or IOTA indices.
 An important detail to note is the previousMessageId:
 This is an important field as it links the new DID Document to the old DID Document, creating a chain.
 Without setting this value, the new DID Document won't get used during resolution of the DID!

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function manipulateIdentity(clientConfig) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // Creates a new identity (see "create_did" example)
    let {key, doc, receipt} = await createIdentity(clientConfig);

    // Add a new VerificationMethod with a new KeyPair
    const newKey = new KeyPair(KeyType.Ed25519);
    const method = new VerificationMethod(doc.id(), newKey.type(), newKey.public(), "newKey");
    doc.insertMethod(method, MethodScope.verificationMethod());

    // Add a new ServiceEndpoint
    const service = new Service({
        id: doc.id().toUrl().join("#linked-domain"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org",
    });
    doc.insertService(service);

    /*
        Add the messageId of the previous message in the chain.
        This is REQUIRED in order for the messages to form a chain.
        Skipping / forgetting this will render the publication useless.
    */
    doc.setMetadataPreviousMessageId(receipt.messageId());
    doc.setMetadataUpdated(Timestamp.nowUTC());

    // Sign the DID Document with the appropriate key.
    doc.signSelf(key, doc.defaultSigningMethod().id());

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const updateReceipt = await client.publishDocument(doc);

    // Log the results.
    console.log(`DID Document Update Transaction: ${clientConfig.explorer.messageUrl(updateReceipt.messageId())}`);
    console.log(`Explore the DID Document: ${clientConfig.explorer.resolverUrl(doc.id())}`);

    return {
        key,
        newKey,
        doc,
        originalMessageId: receipt.messageId(),
        updatedMessageId: updateReceipt.messageId(),
    };
}

export {manipulateIdentity};
