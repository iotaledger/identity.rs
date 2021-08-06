import { getExplorerUrl, logExplorerUrlToScreen, logObjectToScreen, logToScreen } from "./utils.js";
import * as identity from "../../web/identity_wasm.js";
import { createIdentity } from "./create_did.js";
import { manipulateIdentity } from "./mainpulate_did.js";

/*


    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the events to the output window
*/
export async function createDiff(clientConfig, log = true) {
  // if (log) logToScreen("creating identity...");

  // //Creates a new identity (See "create_did" example)
  // let { key, doc, messageId } = await createIdentity(clientConfig, false);

  // if (log) logObjectToScreen(doc);
  // if (log) logToScreen("manipulating identity...");

  // // Create a default client configuration from the parent config network.
  // const config = identity.Config.fromNetwork(clientConfig.network);

  // // Create a client instance to publish messages to the Tangle.
  // const client = identity.Client.fromConfig(config);

  // //Add a new VerificationMethod with a new KeyPair
  // const newKey = new identity.KeyPair(identity.KeyType.Ed25519);
  // const method = identity.VerificationMethod.fromDID(doc.id, newKey, "newKey");
  // doc.insertMethod(method, "VerificationMethod");

  // //Add a new ServiceEndpoint
  // let serviceJSON = {
  //   id: doc.id + "#linked-domain",
  //   type: "LinkedDomains",
  //   serviceEndpoint: "https://iota.org",
  // };
  // doc.insertService(identity.Service.fromJSON(serviceJSON));

  // /*
  //       Add the messageId of the previous message in the chain.
  //       This is REQUIRED in order for the messages to form a chain.
  //       Skipping / forgetting this will render the publication useless.
  //   */
  // doc.previousMessageId = messageId;

  // // Sign the DID Document with the appropriate key.
  // doc.sign(key);

  // // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
  // const nextMessageId = await client.publishDocument(doc.toJSON());

  // if (log) logObjectToScreen(doc);

  // const explorerUrl = getExplorerUrl(doc, nextMessageId);
  // if (log) logExplorerUrlToScreen(explorerUrl);



  
  if (log) logToScreen("creating diff message");

  // Create a default client configuration from network.
  const config = identity.Config.fromNetwork(clientConfig.network);

  // Create a client instance to publish messages to the Tangle.
  const client = identity.Client.fromConfig(config);

  // Creates a new identity, that also is updated (See "manipulate_did" example).
  const {doc, key, messageIdOfSecondMessage} = await manipulateIdentity(clientConfig, false);



  // Create a DID Document (an identity).
  const doc2 = identity.Document.fromJSON(doc.toJSON())

  console.log(doc2);


  //Add a new ServiceEndpoint
  let serviceJSON = {
    id: doc.id + "#new-linked-domain",
    type: "new-LinkedDomains",
    serviceEndpoint: "https://identity.iota.org",
  };
  doc2.insertService(identity.Service.fromJSON(serviceJSON));


  //create diff
  const diff = doc.diff(doc2, messageIdOfSecondMessage, key);
  console.log(diff);

  const diffMessagegId = await client.publishDiff(messageIdOfSecondMessage, diff);
  console.log(diffMessagegId);


  console.log(diff);

  logToScreen(">>>>>>>>>");

  const explorerUrl2 = getExplorerUrl(doc, diffMessagegId);
  if (log) logExplorerUrlToScreen(explorerUrl2);

  return { key, newKey, doc, nextMessageId, explorerUrl };
}
