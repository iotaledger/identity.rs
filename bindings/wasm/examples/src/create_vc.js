// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Credential, CredentialValidator, SignatureOptions, CredentialValidationOptions, FailFast} from '@iota/identity-wasm';
import {createIdentity} from './create_did';
import {manipulateIdentity} from './manipulate_did';

/**
 This example shows how to create a Verifiable Credential and validate it.
 In this example, alice takes the role of the subject, while we also have an issuer.
 The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
 This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whomever they please.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function createVC(clientConfig) {
    // Creates new identities (See "create_did" and "manipulate_did" examples)
    const alice = await createIdentity(clientConfig);
    const issuer = await manipulateIdentity(clientConfig);

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: alice.doc.id.toString(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = Credential.extend({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.doc.id.toString(),
        credentialSubject,
    });

    // Sign the credential with the Issuer's newKey
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: issuer.doc.id.toString() + "#newKey",
        public: issuer.newKey.public,
        private: issuer.newKey.private,
    }, SignatureOptions.default());

    // Before sending this credential to the holder the issuer wants to validate that some properties
    // of the credential satisfy their expectations.


    // Validate the credential's signature, the credential's semantic structure,
    // check that the issuance date is not in the future and that the expiration date is not in the past.
    CredentialValidator.validate(
        signedVc,
        issuer.doc,
        CredentialValidationOptions.default(),
        FailFast.AllErrors
    );

    // Since `validate` did not throw any errors we know that the credential was successfully validated.
    console.log(`VC successfully validated`);

    // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    const credentialJSON = signedVc.toJSON();
    return {alice, issuer, credentialJSON};
}

export {createVC};
