// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    KeyPair,
    KeyType,
    Client,
    Config,
    Document,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod,
    Network,
} from '@iota/identity-wasm';

/**
 * This script generates an example DID with multiple updates to be used in the Identity Resolver in the IOTA Explorer
 * 
 */
(async function () {
    // Use the Mainnet Tangle network.
    const network = Network.mainnet();

    // Create a default client configuration for the network.
    const config = Config.fromNetwork(network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Generate a new ed25519 public/private key pair.
    const key = new KeyPair(KeyType.Ed25519);

    // Create a DID Document (an identity) from the generated key pair.
    const doc = new Document(key, network.toString());

    // Sign the DID Document with the generated key.
    doc.signSelf(key, doc.defaultSigningMethod().id.toString());

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc);

    console.log(doc.id.toString());


    // ===========================================================================
    // Integration Chain Update 1
    // ===========================================================================

    console.log("start integration update 1");

    // Prepare an integration chain update, which writes the full updated DID document to the Tangle.
    const intDoc1 = Document.fromJSON(doc.toJSON()); // clone the Document

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    const keys1 = new KeyPair(KeyType.Ed25519);
    const method1 = VerificationMethod.fromDID(intDoc1.id, keys1, "keys-1");
    intDoc1.insertMethod(method1, MethodScope.VerificationMethod());

    // Add the `messageId` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    intDoc1.metadataPreviousMessageId = receipt.messageId;
    intDoc1.metadataUpdated = Timestamp.nowUTC();

    // Sign the DID Document with the original private key.
    intDoc1.signSelf(key, intDoc1.defaultSigningMethod().id.toString());

    // Publish the updated DID Document to the Tangle, updating the integration chain.
    // This may take a few seconds to complete proof-of-work.
    const intReceipt1 = await client.publishDocument(intDoc1);


    // ===========================================================================
    // Spam on first integration update.
    // ===========================================================================

    const intIndex = intDoc1.integrationIndex();
    await client.publishJSON(intIndex, { "intSpam:1": true });
    await client.publishJSON(intIndex, { "intSpam:2": true });

    // ===========================================================================
    // Integration Chain Update 2
    // ===========================================================================

    console.log("start integration update 2");

    // Prepare an integration chain update, which writes the full updated DID document to the Tangle.
    const intDoc2 = Document.fromJSON(intDoc1.toJSON()); // clone the Document

    // Add a new VerificationMethod with a new KeyPair, with the tag "keys-1"
    const keys2 = new KeyPair(KeyType.Ed25519);
    const method2 = VerificationMethod.fromDID(intDoc2.id, keys2, "keys-2");
    intDoc2.insertMethod(method2, MethodScope.VerificationMethod());

    // Add the `messageId` of the previous message in the chain.
    // This is REQUIRED in order for the messages to form a chain.
    // Skipping / forgetting this will render the publication useless.
    intDoc2.metadataPreviousMessageId = intReceipt1.messageId;
    intDoc2.metadataUpdated = Timestamp.nowUTC();

    // Sign the DID Document with the original private key.
    intDoc2.signSelf(key, intDoc2.defaultSigningMethod().id.toString());

    // Publish the updated DID Document to the Tangle, updating the integration chain.
    // This may take a few seconds to complete proof-of-work.
    const intReceipt2 = await client.publishDocument(intDoc2);


    // ===========================================================================
    // Diff Chain Update 1
    // ===========================================================================

    console.log("start diff update 1");


    // Prepare a diff chain DID Document update.
    const diffDoc1 = Document.fromJSON(intDoc2.toJSON()); // clone the Document

    // Add a new Service with the tag "linked-domain-1"
    let serviceJSON1 = {
        id: diffDoc1.id + "#linked-domain-1",
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org",
    };
    diffDoc1.insertService(Service.fromJSON(serviceJSON1));
    diffDoc1.metadataUpdated = Timestamp.nowUTC();

    // Create a signed diff update.
    //
    // This is the first diff so the `previousMessageId` property is
    // set to the last DID document published on the integration chain.
    const diff1 = intDoc2.diff(diffDoc1, intReceipt2.messageId, key, intDoc2.defaultSigningMethod().id.toString());

    // Publish the diff to the Tangle, starting a diff chain.
    const diffReceipt1 = await client.publishDiff(intReceipt2.messageId, diff1);


    // ===========================================================================
    // Spam on first diff update.
    // ===========================================================================

    let diffIndex = Document.diffIndex(intReceipt2.messageId);
    await client.publishJSON(diffIndex, { "diffSpam:1": true });
    await client.publishJSON(diffIndex, { "diffSpam:2": true });

    // ===========================================================================
    // Diff Chain Update 2
    // ===========================================================================

    console.log("start diff update 2");

    // Prepare another diff chain update.
    const diffDoc2 = Document.fromJSON(diffDoc1.toJSON());

    // Add a second Service with the tag "linked-domain-2"
    let serviceJSON2 = {
        id: diffDoc2.id + "#linked-domain-2",
        type: "LinkedDomains",
        serviceEndpoint: {
            "origins": ["https://iota.org/", "https://example.com/"]
        },
    };
    diffDoc2.insertService(Service.fromJSON(serviceJSON2));
    diffDoc2.metadataUpdated = Timestamp.nowUTC();

    // This is the second diff therefore its `previousMessageId` property is
    // set to the first published diff to extend the diff chain.
    const diff2 = diffDoc1.diff(diffDoc2, diffReceipt1.messageId, key, diffDoc1.defaultSigningMethod().id.toString());

    // Publish the diff to the Tangle.
    // Note that we still use the `messageId` from the last integration chain message here to link
    // the current diff chain to that point on the integration chain.
    const diffReceipt2 = await client.publishDiff(intReceipt2.messageId, diff2);



    // ===========================================================================
    // Diff Chain Update 3
    // ===========================================================================

    console.log("start diff update 3");

    // Prepare another diff chain update.
    const diffDoc3 = Document.fromJSON(diffDoc2.toJSON());

    // Add a second Service with the tag "linked-domain-2"
    let serviceJSON3 = {
        id: diffDoc3.id + "#linked-domain-3",
        type: "LinkedDomains",
        serviceEndpoint: ["https://iota.org/", "https://example.com/"],
    };
    diffDoc3.insertService(Service.fromJSON(serviceJSON3));
    diffDoc3.metadataUpdated = Timestamp.nowUTC();

    // This is the second diff therefore its `previousMessageId` property is
    // set to the first published diff to extend the diff chain.
    const diff3 = diffDoc2.diff(diffDoc3, diffReceipt2.messageId, key, diffDoc2.defaultSigningMethod().id.toString());

    // Publish the diff to the Tangle.
    // Note that we still use the `messageId` from the last integration chain message here to link
    // the current diff chain to that point on the integration chain.
    const diffReceipt3 = await client.publishDiff(intReceipt2.messageId, diff3);


    // ===========================================================================
    // Integration Chain Update 3
    // ===========================================================================
    console.log("start integration update 3");

    // Publish a second integration chain update
    let intDoc3 = Document.fromJSON(diffDoc3.toJSON());

    // Remove the #keys-1 VerificationMethod
    intDoc3.removeMethod(intDoc3.id.toUrl().join("#keys-1"));

    // Remove the #linked-domain-1 Service
    intDoc3.removeService(intDoc3.id.toUrl().join("#linked-domain-1"));

    // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    //       update, NOT the last diff chain message.
    intDoc3.metadataPreviousMessageId = intReceipt2.messageId;
    intDoc3.metadataUpdated = Timestamp.nowUTC();
    intDoc3.signSelf(key, intDoc3.defaultSigningMethod().id.toString());
    const intReceipt3 = await client.publishDocument(intDoc3);

    console.log("done!");

})();
