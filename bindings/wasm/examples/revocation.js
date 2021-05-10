// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { DID, checkCredential, publish } = require('../node/identity_wasm')
const { createVC } = require('./create_VC');
const { EXPLORER_URL, CLIENT_CONFIG } = require('./config')

/*
    This example shows how to revoke a verifiable credential.
    The Verifiable Credential is revoked by actually removing a verification method (public key) from the DID Document of the Issuer.
    As such, the Verifiable Credential can no longer be validated.
    This would invalidate every Verifiable Credential signed with the same public key, therefore the issuer would have to sign every VC with a different key.
    Have a look at the Merkle Key example on how to do that practically.
*/
async function revoke() {
    //Creates new identities (See "create_did" and "manipulate_did" examples)
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
