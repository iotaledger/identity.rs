// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { VerifiablePresentation, checkPresentation } = require('../node/identity_wasm')
const { createVC } = require('./create_VC');
const { CLIENT_CONFIG } = require('./config')

async function createVP() {
    // Creates a new identity (See "create_did" and "manipulate_did" examples)
    const {alice, issuer, signedVc} = await createVC();

    // Create a Verifiable Presentation from the Credential - signed by Alice's key
    // TODO: Sign with a challenge
    const unsignedVp = new VerifiablePresentation(alice.doc, signedVc.toJSON())
    
    const signedVp = alice.doc.signPresentation(unsignedVp, {
        method: "#key",
        secret: alice.key.secret,
    })

    // Check the validation status of the Verifiable Presentation
    const result = await checkPresentation(signedVp.toString(), CLIENT_CONFIG);
    console.log(`VP verification result: ${result.verified}`);
}

exports.createVP = createVP;