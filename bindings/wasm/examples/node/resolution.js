// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client, Config } = require('../../node/identity_wasm')
const { manipulateIdentity } = require("./manipulate_did");

/**
    A short example to show how to resolve a DID. This returns the latest DID Document.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
**/
async function resolution(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const result = await manipulateIdentity(clientConfig);

    // Resolve a DID.
    return await client.resolve(result.doc.id.toString());
}

exports.resolution = resolution;
