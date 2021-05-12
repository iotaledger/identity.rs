// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { resolve } = require('../node/identity_wasm')
const { manipulateIdentity } = require('./manipulate_did');
const { CLIENT_CONFIG } = require('./config');

/*
    A short example to show how to resolve a DID. This returns the latest DID Document.
*/
async function resolution() {
    //Creates a new identity, that also is updated (See "manipulate_did" example)
    const result = await manipulateIdentity();

    //Resolve a DID
    return await resolve(result.doc.id.toString(), CLIENT_CONFIG);
}

exports.resolution = resolution;
