import {getExplorerUrl, logExplorerUrlToScreen, logObjectToScreen, logToScreen} from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import {createIdentity} from "./create_did.js";

/**
 This example is a basic introduction to creating a diff message and publishing it to the tangle.
 1. A did document is created and published with one service.
 2. The document is cloned and another service is added.
 3. The difference between the two documents is created and published as a diff message.
 4. The final DID will contain both services.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 @param {boolean} log log the events to the output window
 **/
export async function createDiff(clientConfig, log = true) {
    if (log) logToScreen("Creating diff chain ...");

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Create a new identity (see "create_did.js" example).
    const {key, doc, receipt} = await createIdentity(clientConfig, false);

    // Clone the Document
    const updatedDoc = identity.Document.fromJSON(doc.toJSON());

    // Add a Service
    let serviceJSON = {
        id: doc.id + "#new-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    updatedDoc.insertService(identity.Service.fromJSON(serviceJSON));

    // Create diff
    const diff = doc.diff(updatedDoc, receipt.messageId, key);

    if (log) logToScreen("Diff:");
    if (log) logObjectToScreen(diff);

    // Publish the diff to the Tangle
    const diffReceipt = await client.publishDiff(receipt.messageId, diff);
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, diffReceipt.messageId));

    return {updatedDoc, key, diffMessageId: diffReceipt.messageId};
}
