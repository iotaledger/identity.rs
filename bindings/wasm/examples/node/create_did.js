// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client, Config, Document, KeyType } = require('../../node/identity_wasm')
const { logExplorerUrl } = require('./explorer_util')

/**
    This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle.
    A ED25519 Keypair is generated, from which the public key is hashed, becoming the DID.
    The keypair becomes part of the DID Document in order to prove a link between the DID and the published DID Document.
    That same keypair should be used to sign the original DID Document.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
**/
async function createIdentity(clientConfig) {
    // Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519, clientConfig.network.toString());

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc.toJSON());

    // Log the results.
    logExplorerUrl("Identity Creation:", clientConfig.network.toString(), receipt.messageId);

    // Return the results.
    return {key, doc, receipt};
}

exports.createIdentity = createIdentity;
