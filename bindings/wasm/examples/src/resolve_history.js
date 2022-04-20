// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    Document,
    KeyPair,
    KeyType,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod
} from '@iota/identity-wasm';
import {createIdentity} from "./create_did";

/**
 Advanced example that performs multiple updates and demonstrates how to resolve the DID Document history to view them.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function resolveHistory(clientConfig) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // ===========================================================================
    // DID Creation
    // ===========================================================================

    // Create a new identity (see "create_did.js" example).
    const {doc, key, receipt: originalReceipt} = await createIdentity(clientConfig);

    // ===========================================================================
    // Integration Chain Spam
    // ===========================================================================

    // Publish several spam messages to the same index as the integration chain on the Tangle.
    // These are not valid DID documents and are simply to demonstrate that invalid messages can be
    // included in the history, potentially for debugging invalid DID documents.
    const intIndex = doc.integrationIndex();
    await client.publishJSON(intIndex, {"intSpam:1": true});
    await client.publishJSON(intIndex, {"intSpam:2": true});
    await client.publishJSON(intIndex, {"intSpam:3": true});
    await client.publishJSON(intIndex, {"intSpam:4": true});
    await client.publishJSON(intIndex, {"intSpam:5": true});

    // ===========================================================================
    // Integration Chain Update 1
    // ===========================================================================

    // Prepare an integration chain update, which writes the full updated DID document to the Tangle.
    const intDoc1 = doc.clone();

    // Add a new Service with the tag "linked-domain-1"
    const service1 = new Service({
        id: intDoc1.id().toUrl().join("#linked-domain-1"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org",
    });
    intDoc1.insertService(service1);

    // Add a second Service with the tag "linked-domain-2"
    const service2 = new Service({
        id: intDoc1.id().toUrl().join("#linked-domain-2"),
        type: "LinkedDomains",
        serviceEndpoint: {
            "origins": ["https://iota.org/", "https://example.com/"]
        },
    });
    intDoc1.insertService(service2);

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    const keys1 = new KeyPair(KeyType.Ed25519);
    const method1 = new VerificationMethod(intDoc1.id(), keys1.type(), keys1.public(), "keys-1");
    intDoc1.insertMethod(method1, MethodScope.VerificationMethod());

    // Add the `messageId` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    intDoc1.setMetadataPreviousMessageId(originalReceipt.messageId());
    intDoc1.setMetadataUpdated(Timestamp.nowUTC());

    // Sign the DID Document with the original private key.
    intDoc1.signSelf(key, intDoc1.defaultSigningMethod().id());

    // Publish the updated DID Document to the Tangle, updating the integration chain.
    // This may take a few seconds to complete proof-of-work.
    const intReceipt1 = await client.publishDocument(intDoc1);

    // Log the results.
    console.log(`Int. Chain Update (1): ${clientConfig.explorer.messageUrl(intReceipt1.messageId())}`);

    // ===========================================================================
    // DID History 1
    // ===========================================================================

    // Retrieve the message history of the DID.
    const history1 = await client.resolveHistory(doc.id());

    // The history shows two documents in the integration chain.
    console.log(`History (1): ${JSON.stringify(history1, null, 2)}`);

    // ===========================================================================
    // Integration Chain Update 2
    // ===========================================================================

    // Publish a second integration chain update
    let intDoc2 = Document.fromJSON(intDoc1.toJSON());

    // Remove the #keys-1 VerificationMethod
    intDoc2.removeMethod(intDoc2.id().toUrl().join("#keys-1"));

    // Remove the #linked-domain-1 Service
    intDoc2.removeService(intDoc2.id().toUrl().join("#linked-domain-1"));

    // Add a VerificationMethod with a new KeyPair, called "keys-2"
    const keys2 = new KeyPair(KeyType.Ed25519);
    const method2 = new VerificationMethod(intDoc2.id(), keys2.type(), keys2.public(), "keys-2");
    intDoc2.insertMethod(method2, MethodScope.VerificationMethod());

    // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    //       update.
    intDoc2.setMetadataPreviousMessageId(intReceipt1.messageId());
    intDoc2.setMetadataUpdated(Timestamp.nowUTC());
    intDoc2.signSelf(key, intDoc2.defaultSigningMethod().id());
    const intReceipt2 = await client.publishDocument(intDoc2);

    // Log the results.
    console.log(`Int. Chain Update (2): ${clientConfig.explorer.messageUrl(intReceipt2.messageId())}`);

    // ===========================================================================
    // DID History 2
    // ===========================================================================

    // Retrieve the updated message history of the DID.
    const history2 = await client.resolveHistory(doc.id());

    // The history now shows three documents in the integration chain.
    console.log(`History (2): ${JSON.stringify(history2, null, 2)}`);
}

export {resolveHistory};
