// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { DID, checkCredential, publish } = require('../node/identity_wasm')
const { createVC } = require('./create_VC');
const { EXPLORER_URL, CLIENT_CONFIG } = require('./config')

async function revoke() {
    //Creates a new identity (See "create_did" and "manipulate_did" examples)
    const {alice, issuer, signedVc} = await createVC();

    //Remove the public key that signed the VC - effectively revoking the VC as it will no longer be able to verify
    issuer.doc.removeMethod(DID.parse(issuer.doc.id.toString()+"#newKey"));
    issuer.doc.previousMessageId = issuer.nextMessageId;
    issuer.doc.sign(issuer.key);
    const MessageId = await publish(issuer.doc, CLIENT_CONFIG);

    //Log the resulting Identity update
    console.log(`Identity Update: ${EXPLORER_URL}/${MessageId}`);

    //Check the verifiable credential
    const result = await checkCredential(signedVc.toString(), CLIENT_CONFIG);
    console.log(`VC verification result: ${result.verified}`);
}

exports.revokeVC = revoke;