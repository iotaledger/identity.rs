import * as identity from "../../web/identity_wasm.js";
import {
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
    getExplorerUrl,
} from "./utils.js";

/**
    This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle.
    A ED25519 Keypair is generated, from which the public key is hashed, becoming the DID.
    The keypair becomes part of the DID Document in order to prove a link between the DID and the published DID Document.
    That same keypair should be used to sign the original DID Document.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function createIdentity(clientConfig, log = true) {
    if (log) logToScreen("Identity creation started...");
    if (log)
        logToScreen("This might take a few seconds to complete proof of work!");

    // Create a DID Document (an identity).
    const { doc, key } = new identity.Document(
        identity.KeyType.Ed25519,
        clientConfig.network.toString()
    );

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Create a default client configuration from the parent config network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc.toJSON());

    const explorerUrl = getExplorerUrl(doc, receipt.messageId);

    if (log) logToScreen("Identity creation done!");
    if (log) logObjectToScreen(doc);
    if (log) logExplorerUrlToScreen(explorerUrl);

    return { key, doc, receipt, explorerUrl };
}
