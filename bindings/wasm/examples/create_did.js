// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Document, KeyType, publish } = require('../node/identity_wasm')
const { logExplorerUrl } = require('./explorer_util')

/*
    This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle.
    A ED25519 Keypair is generated, from which the public key is hashed, becoming the DID.
    The keypair becomes part of the DID Document in order to prove a link between the DID and the published DID Document.
    That same keypair should be used to sign the original DID Document.
*/
async function createIdentity(clientConfig) {
    // Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519, clientConfig.network);

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const messageId = await publish(doc.toJSON(), clientConfig);

    // Log the results.
    logExplorerUrl("Identity Creation:", clientConfig.network, messageId);

    // Return the results.
    return {key, doc, messageId};
}

exports.createIdentity = createIdentity;
