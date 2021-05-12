// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { VerifiableCredential, checkCredential } = require('../node/identity_wasm')
const { createIdentity } = require('./create_did');
const { manipulateIdentity } = require('./manipulate_did');
const { CLIENT_CONFIG } = require('./config')

/*
    This example shows how to create a Verifiable Credential and validate it.
    In this example, alice takes the role of the subject, while we also have an issuer.
    The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
    This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whoever they please.
*/
async function createVC() {
    //Creates new identities (See "create_did" and "manipulate_did" examples)
    const alice = await createIdentity();
    const issuer = await manipulateIdentity();

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: alice.doc.id.toString(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = VerifiableCredential.extend({
        id: "http://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.doc.id.toString(),
        credentialSubject,
    });

    //Sign the credential with the Issuer's newKey
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: issuer.doc.id.toString()+"#newKey",
        public: issuer.newKey.public,
        secret: issuer.newKey.secret,
    });

    //Check if the credential is verifiable
    const result = await checkCredential(signedVc.toString(), CLIENT_CONFIG);
    console.log(`VC verification result: ${result.verified}`);

    return {alice, issuer, signedVc};
}

exports.createVC = createVC;
