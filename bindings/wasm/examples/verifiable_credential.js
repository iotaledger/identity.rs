// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Document, KeyPair, KeyType, publish, VerificationMethod, Service } = require('../node/identity_wasm')
const { createIdentity } = require('./create_did');
const { CLIENT_CONFIG, EXPLORER_URL } = require('./config')

async function main() {
    //Creates a new identity (See "create_did" example)
    const { key, doc } = await createIdentity();

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: "did:iota:",
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Issue an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = VerifiableCredential.extend({
        id: "http://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: user2.doc.id.toString(),
        credentialSubject,
    })

}


main().then(() => {
    console.log("Ok")
}).catch((error) => {
    console.log("Err >", error)
})