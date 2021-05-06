// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { KeyPair, KeyType, publish, VerificationMethod, Service } = require('../node/identity_wasm')
const { createIdentity } = require('./create_did');
const { CLIENT_CONFIG, EXPLORER_URL } = require('./config');

async function manipulateIdentity() {
    //Creates a new identity (See "create_did" example)
    let { key, doc } = await createIdentity();

    //Add a new VerificationMethod 
    const newKey = new KeyPair(KeyType.Ed25519);
    const method = VerificationMethod.fromDID(doc.id, newKey, "newKey");
    doc.insertMethod(method, "VerificationMethod");

    //Add a new ServiceEndpoint
    const serviceJSON = {
        "id":doc.id+"#linked-domain",
        "type": "LinkedDomains",
        "serviceEndpoint" : "https://iota.org"
    };
    const service = Service.fromJSON(serviceJSON);
    doc.insertService(service);

    //Sign the DID Document with the appropriate key
    console.log(doc.messageId);
    //doc = {previous_message_id: doc.messageId, ...doc.toJSON()}
    doc.sign(key);

    //Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const messageId = await publish(doc.toJSON(), CLIENT_CONFIG);

    //Log the results
    console.log(`Identity Update: ${EXPLORER_URL}/${messageId}`);
    return {key, doc};
}

exports.manipulateIdentity = manipulateIdentity;