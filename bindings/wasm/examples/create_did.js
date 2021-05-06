// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Document, KeyType, publish } = require('../node/identity_wasm')
const { CLIENT_CONFIG, EXPLORER_URL } = require('./config')

//Using this allows us to have an async call (await)
async function createIdentity() {
    //Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519)

    //Sign the DID Document with the generated key
    doc.sign(key);

    //Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const messageId = await publish(doc.toJSON(), CLIENT_CONFIG);

    //Log the results
    console.log(`Identity Creation: ${EXPLORER_URL}/${messageId}`);

    //Return the results
    return {key, doc};
}

exports.createIdentity = createIdentity;