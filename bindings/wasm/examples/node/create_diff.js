// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client, Config, Document, Service } = require("../../node/identity_wasm");
const { manipulateIdentity } = require("./manipulate_did");
const { CLIENT_CONFIG } = require("./config");

/*
    This example is a baisc introduction to creating a diff message and publishing it to the tangle. 
    1. A did document is created and published with one service.
    2. The document is cloned and another service is added.
    3. The difference between the two documents is created and published as a diff message.
    4. The final DID will contain both services.

    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the events to the output window
*/
async function createDiff(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const { doc, key, messageIdOfSecondMessage } = await manipulateIdentity(clientConfig);

    // clone the Document
    const doc2 = Document.fromJSON(doc.toJSON());

    //Add a second ServiceEndpoint
    let serviceJSON = {
        id: doc.id + "#new-linked-domain",
        type: "new-LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };

    doc2.insertService(Service.fromJSON(serviceJSON));
    console.log(doc2);

    //create diff
    const diff = doc.diff(doc2, messageIdOfSecondMessage, key);
    console.log(diff);

    const diffRes = await client.publishDiff(messageIdOfSecondMessage, diff);
    console.log(diffRes);

    return { doc2, key, diffMessageId: diffRes.messageId };
}

exports.createDiff = createDiff;
