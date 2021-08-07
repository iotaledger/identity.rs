import { getExplorerUrl, logExplorerUrlToScreen, logObjectToScreen, logToScreen } from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import { manipulateIdentity } from "./mainpulate_did.js";

/*
    This example is a baisc introduction to creating a diff message and publishing it to the tangle. 
    1. A did document is created and published with one service.
    2. The document is cloned and another service is added.
    3. The difference between the two documents is created and published as a diff message.
    4. The final DID will contain both services.

    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the events to the output window
*/
export async function createDiff(clientConfig, log = true) {
    if (log) logToScreen("creating diff message ...");

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const { doc, key, messageIdOfSecondMessage } = await manipulateIdentity(clientConfig, false);

    // clone the DID 
    const doc2 = identity.Document.fromJSON(doc.toJSON());

    //Add a second ServiceEndpoint
    let serviceJSON = {
        id: doc.id + "#new-linked-domain",
        type: "new-LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    doc2.insertService(identity.Service.fromJSON(serviceJSON));

    //create diff
    const diff = doc.diff(doc2, messageIdOfSecondMessage, key);

    if (log) logToScreen("Diff object:");
    if (log) logObjectToScreen(diff);

    const diffRes = await client.publishDiff(messageIdOfSecondMessage, diff);

    return { doc, key, diffMessageId: diffRes.messageId };
}
