import {getExplorerUrl, logExplorerUrlToScreen, logObjectToScreen, logToScreen} from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import {manipulateIdentity} from "./mainpulate_did.js";

/**
 Advanced example that performs multiple diff chain and integration chain updates and
 demonstrates how to resolve the DID Document history to view these chains.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 @param {boolean} log log the events to the output window
 **/
export async function resolveHistory(clientConfig, log = true) {
    if (log) logToScreen("Resolve History Example");

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // ===========================================================================
    // DID Creation + Integration Chain Update 1
    // ===========================================================================

    // Creates a new identity and performs one integration chain update (See "manipulate_did" example).
    const {doc, key, updatedMessageId} = await manipulateIdentity(clientConfig, false);

    // Publish some unrelated spam messages to the same index as the integration chain on the Tangle.
    // These are not valid DID messages and are simply to demonstrate that invalid messages
    // can be included in the history for debugging invalid DID documents.
    const integration_index = doc.integrationAddress();
    await client.publishJSON(integration_index, key);
    await client.publishJSON(integration_index, {"spam:1": true});
    await client.publishJSON(integration_index, {"spam:2": true});
    await client.publishJSON(integration_index, {"spam:3": true});
    await client.publishJSON(integration_index, {"spam:4": true});
    await client.publishJSON(integration_index, {"spam:5": true});

    // ===========================================================================
    // Diff Chain Update 1
    // ===========================================================================
    // Clone the Document
    const doc2 = identity.Document.fromJSON(doc.toJSON());

    // Add a second ServiceEndpoint
    let serviceJSON2 = {
        id: doc.id + "#new-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://identity.iota.org",
    };
    doc2.insertService(identity.Service.fromJSON(serviceJSON2));

    // Create a signed diff update.
    //
    // This is the first diff therefore the `previous_message_id` property is
    // set to the last DID document published.
    const diff1 = doc.diff(doc2, updatedMessageId, key);

    // Publish the diff to the Tangle, starting a diff chain.
    const diff1Receipt = await client.publishDiff(updatedMessageId, diff1);
    if (log) logToScreen("Diff Chain Update (1):");
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, diff1Receipt.messageId));

    // ===========================================================================
    // Diff Chain Update 2
    // ===========================================================================

    // Add a third ServiceEndpoint
    let serviceJSON3 = {
        id: doc.id + "#third-linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://fake-domain.org",
    };
    doc2.insertService(identity.Service.fromJSON(serviceJSON3));

    // Create a signed diff update.
    //
    // This is the first diff therefore the `previous_message_id` property is
    // set to the last DID document published.
    const diff2 = doc.diff(doc2, diff1Receipt.messageId, key);

    // Publish the diff to the Tangle, starting a diff chain.
    const diff2Receipt = await client.publishDiff(updatedMessageId, diff2);
    if (log) logToScreen("Diff Chain Update (2):");
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, diff2Receipt.messageId));

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
    if (log) logToScreen("History (1):")
    if (log) logObjectToScreen(history1);

    // ===========================================================================
    // Integration Chain Update 2
    // ===========================================================================

    // Publish an integration chain update, which writes the full updated DID document to the Tangle.
    // Note: the previousMessageId points to the messageId of the last integration chain update,
    //       not the last diff chain message.
    doc2.previousMessageId = updatedMessageId;

    // Sign the DID Document with the appropriate key.
    doc2.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const intChainUpdateReceipt = await client.publishDocument(doc.toJSON());

    // Log the results.
    if (log) logToScreen("Int. Chain Update (2):")
    if (log) logExplorerUrlToScreen(getExplorerUrl(doc, intChainUpdateReceipt.messageId));

    // ===========================================================================
    // DID History 2
    // ===========================================================================

    // Retrieve the updated message history of the DID.
    const history2 = await client.resolveHistory(doc.id.toString());

    // The history now shows two documents in the integration chain (plus the current document), and no
    // diffs in the diff chain. This is because the previous document published included those updates
    // and we have not added any diffs pointing to the latest document.
    if (log) logToScreen("History (2):")
    if (log) logObjectToScreen(history2);

    // ===========================================================================
    // Diff Chain History
    // ===========================================================================

    // Fetch the diff chain of the previous integration chain message.
    // We can still retrieve old diff chains, but they do not affect DID resolution.
    let verificationMethod = doc.authentication();
    let diffSet = await client.resolveDiffs(doc.id.toString(), verificationMethod, updatedMessageId);
    if (log) logToScreen("DiffSet:")
    if (log) logObjectToScreen(diffSet);
}
