// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Document, Service, Timestamp} from '@iota/identity-wasm';
import {createIdentity} from "./create_did";

/**
 This example is a basic introduction to creating a diff message and publishing it to the tangle.
 1. A did document is created and published with one service.
 2. The document is cloned and another service is added.
 3. The difference between the two documents is created and published as a diff message.
 4. The final DID will contain both services.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function createDiff(clientConfig) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // Create a new identity (see "create_did.js" example).
    const {key, doc, receipt} = await createIdentity(clientConfig);

    // Clone the Document
    const updatedDoc = doc.clone();

    // Add a Service
    const service = new Service({
        id: doc.id.toUrl().join("#linked-domain-1"),
        type: "LinkedDomains",
        serviceEndpoint: "https://example.com/",
    });
    updatedDoc.insertService(service);
    updatedDoc.metadataUpdated = Timestamp.nowUTC();
    console.log(updatedDoc);

    // Create diff
    const diff = doc.diff(updatedDoc, receipt.messageId, key, doc.defaultSigningMethod().id);
    console.log(diff);

    // Publish diff to the Tangle
    const diffReceipt = await client.publishDiff(receipt.messageId, diff);
    console.log(diffReceipt);
    console.log(`Diff Chain Transaction: ${clientConfig.explorer.messageUrl(diffReceipt.messageId)}`);
    console.log(`Explore the DID Document: ${clientConfig.explorer.resolverUrl(doc.id)}`);

    return {updatedDoc, key, diffMessageId: diffReceipt.messageId};
}

export {createDiff};
