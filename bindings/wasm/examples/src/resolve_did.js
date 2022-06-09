// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Resolver} from '@iota/identity-wasm';
import {createIdentity} from "./create_did";

/**
 A short example to show how to resolve a DID. This returns the latest DID Document.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function resolveIdentity(clientConfig) {

    // Creates a new identity (see "create_did" example)
    const {key, doc, receipt} = await createIdentity(clientConfig);
    const doc_did = doc.id();

    // Retrieve the published DID Document from the Tangle.
    const resolver = await Resolver
        .builder()
        .clientConfig({
            network: clientConfig.network
        })
        .build();

    return await resolver.resolve(doc_did);
}

export {resolveIdentity};
