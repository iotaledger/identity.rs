// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    Client,
    Document,
    ExplorerUrl,
    KeyPair,
    KeyType,
    MethodContent,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod
} from '@iota/identity-wasm/node';
import {createIdentity} from "../basic/1_create_did";

/**
 Advanced example that performs multiple updates and demonstrates how to resolve the DID Document history to view them.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function resolveHistory() {

    // ===========================================================================
    // DID Creation
    // ===========================================================================
    let builder = new AccountBuilder();
    let account = await builder.createIdentity();

    // Retrieve the DID document of the newly created identity.
    const doc = account.document(); 
    
    // ===========================================================================
    // Integration Chain Spam
    // ===========================================================================
    
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await new Client();

    // Publish several spam messages to the same index as the integration chain on the Tangle.
    // These are not valid DID documents and are simply to demonstrate that invalid messages can be
    // included in the history, potentially for debugging invalid DID documents.
    const intIndex = doc.integrationIndex();
    await client.publishJSON(intIndex, {"intSpam:1": true});
    await client.publishJSON(intIndex, {"intSpam:2": true});
    await client.publishJSON(intIndex, {"intSpam:3": true});
    await client.publishJSON(intIndex, {"intSpam:4": true});
    await client.publishJSON(intIndex, {"intSpam:5": true});

    // ===========================================================================
    // Integration Chain Update 1
    // ===========================================================================

    await account.createService({
        fragment: "linked-domain-1",
        type: "LinkedDomains",
        endpoint: "https://iota.org",
    });

    //TODO: How to add `serviceEndpoint: { "origins": ["https://iota.org/", "https://example.com/"] },`
    await account.createService({
        fragment: "linked-domain-2",
        type: "LinkedDomains",
        endpoint: "https://iota.org",
    });

    await account.createMethod({
        fragment: "keys-1",
        content: MethodContent.GenerateEd25519(),
    });

    // Log the results.
    console.log(`Int. Chain Update (1): ${ExplorerUrl.mainnet().resolverUrl(account.did())}`);

    // ===========================================================================
    // DID History 1
    // ===========================================================================

    // Retrieve the message history of the DID.
    const history1 = await client.resolveHistory(doc.id());

    // The history shows two documents in the integration chain.
    console.log(`History (1): ${JSON.stringify(history1, null, 2)}`);

    // // ===========================================================================
    // // Integration Chain Update 2
    // // ===========================================================================

    // // Publish a second integration chain update
    // let intDoc2 = Document.fromJSON(intDoc1.toJSON());

    // // Remove the #keys-1 VerificationMethod
    // intDoc2.removeMethod(intDoc2.id().toUrl().join("#keys-1"));

    // // Remove the #linked-domain-1 Service
    // intDoc2.removeService(intDoc2.id().toUrl().join("#linked-domain-1"));

    // // Add a VerificationMethod with a new KeyPair, called "keys-2"
    // const keys2 = new KeyPair(KeyType.Ed25519);
    // const method2 = new VerificationMethod(intDoc2.id(), keys2.type(), keys2.public(), "keys-2");
    // intDoc2.insertMethod(method2, MethodScope.VerificationMethod());

    // // Note: the `previous_message_id` points to the `message_id` of the last integration chain
    // //       update.
    // intDoc2.setMetadataPreviousMessageId(intReceipt1.messageId());
    // intDoc2.setMetadataUpdated(Timestamp.nowUTC());
    // intDoc2.signSelf(key, intDoc2.defaultSigningMethod().id());
    // const intReceipt2 = await client.publishDocument(intDoc2);

    // // Log the results.
    // console.log(`Int. Chain Update (2): ${clientConfig.explorer.messageUrl(intReceipt2.messageId())}`);

    // // ===========================================================================
    // // DID History 2
    // // ===========================================================================

    // // Retrieve the updated message history of the DID.
    // const history2 = await client.resolveHistory(doc.id());

    // // The history now shows three documents in the integration chain.
    // console.log(`History (2): ${JSON.stringify(history2, null, 2)}`);
}

export {resolveHistory};
