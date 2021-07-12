import { getExplorerUrl } from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import { createIdentity } from "./create_did.js";

export async function manipulateIdentity() {
    //Creates a new identity (See "create_did" example)
    let { key, doc, messageId } = await createIdentity();

    // Create a default client configuration from the parent config network.
    const config = identity.Config.fromNetwork(doc.id.network);

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
    doc.previousMessageId = messageId;

    // Sign the DID Document with the appropriate key.
    doc.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const nextMessageId = await client.publishDocument(doc.toJSON());

    const explorerUrl = getExplorerUrl(doc, nextMessageId);
    return { key, newKey, doc, nextMessageId, explorerUrl };
}
