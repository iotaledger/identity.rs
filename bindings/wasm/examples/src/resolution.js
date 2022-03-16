// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client} from '@iota/identity-wasm';
import {manipulateIdentity} from "./manipulate_did";

/**
 A short example to show how to resolve a DID. This returns the latest DID Document.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 @param {string} did
 **/
async function resolution(clientConfig, did) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    if (!did) {
        // Creates a new identity, that also is updated (See "manipulate_did" example).
        let {doc} = await manipulateIdentity(clientConfig);
        did = doc.id.toString();
    }

    // Resolve a DID.
    return await client.resolve(did);
}

export {resolution};
