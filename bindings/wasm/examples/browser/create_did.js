import { getExplorerUrl, logToScreen } from "./utils.js";
import * as id from "../../web/identity_wasm.js";

export async function createIdentity() {
  logToScreen("Identity creation started...");

  // Create a DID Document (an identity).
  const { doc, key } = new id.Document(id.KeyType.Ed25519);

  // Sign the DID Document with the generated key.
  doc.sign(key);

  // Create a default client configuration from the parent config network.
  const config = id.Config.fromNetwork(doc.id.network);

  // Create a client instance to publish messages to the Tangle.
  const client = id.Client.fromConfig(config);

  // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
  const messageId = await client.publishDocument(doc.toJSON());

  const explorerUrl = getExplorerUrl(doc, messageId);

  logToScreen("Identity creation done!");
  logToScreen(`Explorer URL: <a target="_blank" href="${explorerUrl}"> ${explorerUrl} </a>`);
}

//run the createIdentity function on button click
document
  .querySelector("#create-identity-btn")
  .addEventListener("click", createIdentity);
