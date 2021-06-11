// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { VerifiablePresentation, checkPresentation } = require('../node/identity_wasm')
const { createVC } = require('./create_VC');

/*
    This example shows how to create a Verifiable Presentation and validate it.
    A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
    It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

    @param {{network: string, node: string}} clientConfig
*/
async function createVP(clientConfig) {
    // Creates new identities (See "createVC" example)
    const {alice, issuer, signedVc} = await createVC(clientConfig);

    // Create a Verifiable Presentation from the Credential - signed by Alice's key
    // TODO: Sign with a challenge
    const unsignedVp = new VerifiablePresentation(alice.doc, signedVc.toJSON())

    const signedVp = alice.doc.signPresentation(unsignedVp, {
        method: "#key",
        secret: alice.key.secret,
    })

    // Check the validation status of the Verifiable Presentation
    const result = await checkPresentation(signedVp.toString(), {
        network: clientConfig.network.toString(),
        node: clientConfig.defaultNodeURL,
    });

    console.log(`VP verification result: ${result.verified}`);
}

exports.createVP = createVP;
