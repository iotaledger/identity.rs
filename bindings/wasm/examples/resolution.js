// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { resolve } = require('../node/identity_wasm')
const { manipulateIdentity } = require('./manipulate_did');
const { CLIENT_CONFIG, EXPLORER_URL } = require('./config');

async function resolution() {
    //Creates a new identity, that also is updated (See "manipulate_did" example)
    const { key, doc } = await manipulateIdentity();

    //Resolve a DID
    return await resolve(doc.id);
}

exports.resolution = resolution;
