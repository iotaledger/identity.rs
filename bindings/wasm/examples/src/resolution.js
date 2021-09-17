// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Config} from '@iota/identity-wasm';
import {manipulateIdentity} from "./manipulate_did";

/**
 A short example to show how to resolve a DID. This returns the latest DID Document.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 @param {string} did
 **/
async function resolution(clientConfig, did) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    if (!did) {
        // Creates a new identity, that also is updated (See "manipulate_did" example).
        let {doc} = await manipulateIdentity(clientConfig);
        did = doc.id.toString();
    }

    // Resolve a DID.
    return await client.resolve(did);
}

export {resolution};
