// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Config, Document, Service} from '@iota/identity-wasm';
import {createIdentity} from "./create_did";
import {logExplorerUrl, logResolverUrl} from "./utils";

/**
 This example is a basic introduction to creating a diff message and publishing it to the tangle.
 1. A did document is created and published with one service.
 2. The document is cloned and another service is added.
 3. The difference between the two documents is created and published as a diff message.
 4. The final DID will contain both services.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function createDiff(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Create a new identity (see "create_did.js" example).
    const {key, doc, receipt} = await createIdentity(clientConfig);

    // Clone the Document
    const updatedDoc = Document.fromJSON(doc.toJSON());

    // Add a Service
    let serviceJSON = {
        id: doc.id + "#new-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    updatedDoc.insertService(Service.fromJSON(serviceJSON));
    console.log(updatedDoc);

    // Create diff
    const diff = doc.diff(updatedDoc, receipt.messageId, key, doc.defaultSigningMethod().id.toString());
    console.log(diff);

    // Publish diff to the Tangle
    const diffReceipt = await client.publishDiff(receipt.messageId, diff);
    console.log(diffReceipt);
    logExplorerUrl("Diff Chain Transaction:", clientConfig.explorer, diffReceipt.messageId);
    logResolverUrl("Explore the DID Document:", clientConfig.explorer, doc.id.toString());

    return {updatedDoc, key, diffMessageId: diffReceipt.messageId};
}

export {createDiff};
