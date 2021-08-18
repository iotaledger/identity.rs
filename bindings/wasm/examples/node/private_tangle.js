// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client, Config, Document, KeyType, Network } = require('../../node/identity_wasm')

/**
    This example shows how a DID document can be created on a private tangle.
    It can be run together with a local hornet node.
    Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
    for setup instructions.
**/
async function createIdentityPrivateTangle() {
    // This is the `networkID` defined in hornet's node configuration.
    const network = Network.from_name("private-tangle");

    // Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519, network.toString());

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Create a client configuration and set the custom network.
    const config = new Config();
    config.setNetwork(network);

    // This URL points to the REST API of the locally running hornet node.
    config.setNode("http://127.0.0.1:14265/");

    // Create a client instance from the configuration to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc.toJSON());

    // Return the results.
    return { key, doc, receipt };
}

exports.createIdentityPrivateTangle = createIdentityPrivateTangle;
