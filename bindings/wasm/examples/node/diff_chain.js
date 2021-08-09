// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { logExplorerUrl } = require("./explorer_util");
const { Client, Config, Document, Service } = require("../../node/identity_wasm");
const { manipulateIdentity } = require("./manipulate_did");

/**
    This example is a basic introduction to creating a diff message and publishing it to the tangle.
    1. A did document is created and published with one service.
    2. The document is cloned and another service is added.
    3. The difference between the two documents is created and published as a diff message.
    4. The final DID will contain both services.

    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the events to the output window
**/
async function createDiffChain(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const { doc, key, updatedMessageId } = await manipulateIdentity(clientConfig);

    // clone the Document
    const doc2 = Document.fromJSON(doc.toJSON());

    // Add a second ServiceEndpoint
    let serviceJSON = {
        id: doc.id + "#new-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    doc2.insertService(Service.fromJSON(serviceJSON));
    console.log(doc2);

    // Create diff update
    const diff = doc.diff(doc2, updatedMessageId, key);
    console.log(diff);

    // Publish diff to the Tangle
    const diffReceipt = await client.publishDiff(updatedMessageId, diff);
    console.log(diffReceipt);
    logExplorerUrl("Diff Chain Transaction:", clientConfig.network.toString(), diffReceipt.messageId);

    return { doc2, key, diffMessageId: diffReceipt.messageId };
}

exports.createDiffChain = createDiffChain;
