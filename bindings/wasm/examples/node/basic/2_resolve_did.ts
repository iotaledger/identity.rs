// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Resolver, AccountBuilder} from '@iota/identity-wasm/node';

/**
 A short example to show how to resolve a DID. This returns the latest DID Document.
 **/
async function resolveDID(storage?: Storage) {

    // Creates a new identity
    let builder = new AccountBuilder({
        storage,
    });
    let account = await builder.createIdentity();
    
    const doc_did = account.did(); 

    // Retrieve the published DID Document from the Tangle. 
    const resolver = await Resolver
    .builder()
    .build();

    return await resolver.resolve(doc_did);
}

export {resolveDID};
