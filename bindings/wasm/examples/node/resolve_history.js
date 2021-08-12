// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const {logExplorerUrl} = require("./explorer_util");
const {Client, Config, Document, Service} = require("../../node/identity_wasm");
const {manipulateIdentity} = require("./manipulate_did");

/**
 Advanced example that performs multiple diff chain and integration chain updates and
 demonstrates how to resolve the DID Document history to view these chains.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 **/
async function resolveHistory(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // ===========================================================================
    // DID Creation + Integration Chain Update 1
    // ===========================================================================

    // Creates a new identity, that also is updated on the integration chain (See "manipulate_did" example).
    const {doc, key, updatedMessageId} = await manipulateIdentity(clientConfig);

    // ===========================================================================
    // Integration Chain Spam
    // ===========================================================================

    // Publish several spam messages to the same index as the integration chain on the Tangle.
    // These are not valid DID documents and are simply to demonstrate that invalid messages can be
    // included in the history, potentially for debugging invalid DID documents.
    const integration_index = doc.integrationAddress();
    await client.publishJSON(integration_index, {"intSpam:1": true});
    await client.publishJSON(integration_index, {"intSpam:2": true});
    await client.publishJSON(integration_index, {"intSpam:3": true});
    await client.publishJSON(integration_index, {"intSpam:4": true});
    await client.publishJSON(integration_index, {"intSpam:5": true});

    // ===========================================================================
    // Diff Chain Update 1
    // ===========================================================================

    // Clone the Document
    const doc2 = Document.fromJSON(doc.toJSON());

    // Add a second ServiceEndpoint
    let serviceJSON2 = {
        id: doc.id + "#new-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    doc2.insertService(Service.fromJSON(serviceJSON2));

    // Create a signed diff update.
    //
    // This is the first diff therefore the `previous_message_id` property is
    // set to the last DID document published.
    const diff1 = doc.diff(doc2, updatedMessageId, key);

    // Publish the diff to the Tangle, starting a diff chain.
    const diff1Receipt = await client.publishDiff(updatedMessageId, diff1);
    logExplorerUrl("Diff Chain Transaction (1):", clientConfig.network.toString(), diff1Receipt.messageId);

    // ===========================================================================
    // Diff Chain Update 2
    // ===========================================================================

    const doc3 = Document.fromJSON(doc2.toJSON());

    // Add a third ServiceEndpoint
    let serviceJSON3 = {
        id: doc3.id + "#third-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://fake-domain.org",
    };
    doc3.insertService(Service.fromJSON(serviceJSON3));

    // Create a signed diff update.
    //
    // This is the first diff therefore the `previous_message_id` property is
    // set to the last DID document published.
    const diff2 = doc2.diff(doc3, diff1Receipt.messageId, key);

    // Publish the diff to the Tangle.
    // Note that we still use the message_id from the last integration chain message here to link
    // the current diff chain to that point on the integration chain.
    const diff2Receipt = await client.publishDiff(updatedMessageId, diff2);
    logExplorerUrl("Diff Chain Transaction (2):", clientConfig.network.toString(), diff2Receipt.messageId);

    // ===========================================================================
    // Diff Chain Spam
    // ===========================================================================

    // Publish several spam messages to the same index as the new diff chain on the Tangle.
    let diffIndex = Document.diffAddress(updatedMessageId);
    await client.publishJSON(diffIndex, { "diffSpam:1": true });
    await client.publishJSON(diffIndex, { "diffSpam:2": true });
    await client.publishJSON(diffIndex, { "diffSpam:3": true });

    // ===========================================================================
    // DID History 1
    // ===========================================================================

    // Retrieve the message history of the DID.
    const history1 = await client.resolveHistory(doc.id.toString());

    // The history shows one document in the integration chain (plus the current document), and two
    // diffs in the diff chain.
    console.log("History 1:");
    console.log(history1);

    // ===========================================================================
    // Integration Chain Update 2
    // ===========================================================================

    // Publish an integration chain update, which writes the full updated DID document to the Tangle.
    // Note: the previousMessageId points to the messageId of the last integration chain update,
    //       not the last diff chain message.
    doc3.previousMessageId = updatedMessageId;

    // Sign the DID Document with the appropriate key.
    doc3.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const intChainUpdateReceipt = await client.publishDocument(doc.toJSON());

    // Log the results.
    logExplorerUrl("Int. Chain Update (2):", clientConfig.network.toString(), intChainUpdateReceipt.messageId);

    // ===========================================================================
    // DID History 2
    // ===========================================================================

    // Retrieve the updated message history of the DID.
    const history2 = await client.resolveHistory(doc.id.toString());

    // The history now shows two documents in the integration chain (plus the current document), and no
    // diffs in the diff chain. This is because the previous document published included those updates
    // and we have not added any diffs pointing to the latest document.
    console.log("History 2:");
    console.log(history2)

    // ===========================================================================
    // Diff Chain History
    // ===========================================================================

    // Fetch the diff chain of the previous integration chain message.
    // We can still retrieve old diff chains, but they do not affect DID resolution.
    let verificationMethod = doc.authentication();
    let diffSet = await client.resolveDiffs(doc.id.toString(), verificationMethod, updatedMessageId);
    console.log("DiffSet:");
    console.log(diffSet);
}

exports.resolveHistory = resolveHistory;
