// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Client, Config } = require('../../node/identity_wasm')
const { manipulateIdentity } = require("./manipulate_did");
const { CLIENT_CONFIG } = require("./config");

/*

    @param {{network: string, node: string}} clientConfig
*/
async function resolveHistory(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const result = await manipulateIdentity(clientConfig);

    console.log(result)
    const chain = await client.resolveHistory(result.doc.id.toString())

    console.log(chain);
    console.log(chain.intChainData.history);
}

resolveHistory(CLIENT_CONFIG)

// exports.resolveHistory = resolveHistory;
