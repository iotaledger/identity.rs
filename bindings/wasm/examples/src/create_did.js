// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Document, KeyPair, KeyType} from '@iota/identity-wasm';

/**
 This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle.
 A ED25519 Keypair is generated, from which the public key is hashed, becoming the DID.
 The keypair becomes part of the DID Document in order to prove a link between the DID and the published DID Document.
 That same keypair should be used to sign the original DID Document.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function createIdentity(clientConfig) {
    // Generate a new ed25519 public/private key pair.
    const key = new KeyPair(KeyType.Ed25519);

    // Create a DID Document (an identity) from the generated key pair.
    const doc = new Document(key, clientConfig.network.name());

    // Sign the DID Document with the generated key.
    doc.signSelf(key, doc.defaultSigningMethod().id());

    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc);

    // Log the results.
    console.log(`DID Document Transaction: ${clientConfig.explorer.messageUrl(receipt.messageId())}`);
    console.log(`Explore the DID Document: ${clientConfig.explorer.resolverUrl(doc.id())}`);

    // Return the results.
    return {key, doc, receipt};
}

export {createIdentity};
