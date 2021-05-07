// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { VerifiableCredential, checkCredential } = require('../node/identity_wasm')
const { createIdentity } = require('./create_did');
const { manipulateIdentity } = require('./manipulate_did');
const { CLIENT_CONFIG } = require('./config')

async function createVC() {
    //Creates a new identity (See "create_did" and "manipulate_did" examples)
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