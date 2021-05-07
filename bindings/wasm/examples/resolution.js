// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { resolve } = require('../node/identity_wasm')
const { manipulateIdentity } = require('./manipulate_did');
const { CLIENT_CONFIG } = require('./config');

async function resolution() {
    //Creates a new identity, that also is updated (See "manipulate_did" example)
    const result = await manipulateIdentity();

    //Resolve a DID (Make sure to provide a string input)
    return await resolve(result.doc.id.toString(), CLIENT_CONFIG);
}

exports.resolution = resolution;