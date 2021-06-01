// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { KeyPair, KeyType, publish, VerificationMethod, Service } = require('../node/identity_wasm')
const { createIdentity } = require('./create_did');
const { logExplorerUrl } = require('./explorer_util');

/*
    This example shows how to add more to an existing DID Document.
    The two main things to add are Verification Methods and Services.
    A verification method adds public keys, which can be used to digitally sign things as an identity.
    The services provide metadata around the identity via URIs. These can be URLs, but can also emails or IOTA indices.
    An important detail to note is the previousMessageId. This is an important field as it links the new DID Document to the old DID Document, creating a chain.
    Without setting this value, the new DID Document won't get used during resolution of the DID!

    @param {{network: string, node: string}} clientConfig
*/
async function manipulateIdentity(clientConfig) {
    //Creates a new identity (See "create_did" example)
    let { key, doc, messageId } = await createIdentity(clientConfig);

    //Add a new VerificationMethod with a new KeyPair
    const newKey = new KeyPair(KeyType.Ed25519);
    const method = VerificationMethod.fromDID(doc.id, newKey, "newKey");
    doc.insertMethod(method, "VerificationMethod");

    //Add a new ServiceEndpoint
    const serviceJSON = {
        "id":doc.id+"#linked-domain",
        "type": "LinkedDomains",
        "serviceEndpoint" : "https://iota.org"
    };
    doc.insertService(Service.fromJSON(serviceJSON));

    /*
        Add the messageId of the previous message in the chain.
        This is REQUIRED in order for the messages to form a chain.
        Skipping / forgetting this will render the publication useless.
    */
    doc.previousMessageId = messageId;

    // Sign the DID Document with the appropriate key.
    doc.sign(key);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const nextMessageId = await publish(doc.toJSON(), clientConfig);

    // Log the results.
    logExplorerUrl("Identity Update:", clientConfig.network, messageId);
    return {key, newKey, doc, nextMessageId};
}

exports.manipulateIdentity = manipulateIdentity;
