import {
    getExplorerUrl,
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
} from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import { createIdentity } from "./create_did.js";

/**
    This example shows how to add more to an existing DID Document.
    The two main things to add are Verification Methods and Services.
    A verification method adds public keys, which can be used to digitally sign things as an identity.
    The services provide metadata around the identity via URIs. These can be URLs, but can also emails or IOTA indices.
    An important detail to note is the previousMessageId. This is an important field as it links the new DID Document to the old DID Document, creating a chain.
    Without setting this value, the new DID Document won't get used during resolution of the DID!

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
    @param {boolean} log log the events to the output window
**/
export async function manipulateIdentity(clientConfig, log = true) {
    if (log) logToScreen("creating identity...");

    //Creates a new identity (See "create_did" example)
    let { key, doc, receipt } = await createIdentity(clientConfig, false);

    if (log) logObjectToScreen(doc);
    if (log) logToScreen("manipulating identity...");

    // Create a default client configuration from the parent config network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    //Add a new VerificationMethod with a new KeyPair
    const newKey = new identity.KeyPair(identity.KeyType.Ed25519);
    const method = identity.VerificationMethod.fromDID(
        doc.id,
        newKey,
        "newKey"
    );
    doc.insertMethod(method, "VerificationMethod");

    //Add a new ServiceEndpoint
    const serviceJSON = {
        id: doc.id + "#linked-domain",
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org",
    };
    doc.insertService(identity.Service.fromJSON(serviceJSON));

    /*
        Add the messageId of the previous message in the chain.
        This is REQUIRED in order for the messages to form a chain.
        Skipping / forgetting this will render the publication useless.
    */
    doc.previousMessageId = receipt.messageId;

    // Sign the DID Document with the appropriate key.
    doc.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const nextReceipt = await client.publishDocument(doc.toJSON());

    if (log) logObjectToScreen(doc);

    const explorerUrl = getExplorerUrl(doc, nextReceipt.messageId);
    if (log) logExplorerUrlToScreen(explorerUrl);

    return { key, newKey, doc, receipt, explorerUrl };
}
