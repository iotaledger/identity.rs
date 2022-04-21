// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    CredentialValidationOptions,
    CredentialValidator,
    FailFast,
    ProofOptions,
    AccountBuilder,
    Storage,
    MethodContent,
} from '@iota/identity-wasm/node';

/**
 This example shows how to create a Verifiable Credential and validate it.
 In this example, alice takes the role of the subject, while we also have an issuer.
 The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
 This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whomever they please.
 **/
async function createVC(storage?: Storage) {
    let builder = new AccountBuilder({
        storage,
    });

    // Create an identity for the issuer.
    let issuer = await builder.createIdentity();

    // Add verification method to the issuer.
    await issuer.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "#issuerKey"
    })

    // Create a credential subject indicating the degree earned by Alice.
    let credentialSubject = {
        id: "did:iota:B8DucnzULJ9E8cmaReYoePU2b7UKE9WKxyEVov8tQA7H",
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = Credential.extend({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.document().id().toString(),
        credentialSubject,
    });

    // Created a signed credential by the issuer. 
    const signedVc = await issuer.createSignedCredential(
        "#issuerKey",
        unsignedVc,
        ProofOptions.default(),
    );


    // Before sending this credential to the holder the issuer wants to validate that some properties
    // of the credential satisfy their expectations.


    // Validate the credential's signature, the credential's semantic structure,
    // check that the issuance date is not in the future and that the expiration date is not in the past.
    CredentialValidator.validate(
        signedVc,
        issuer.document(),
        CredentialValidationOptions.default(),
        FailFast.AllErrors
    );

    // Since `validate` did not throw any errors we know that the credential was successfully validated.
    console.log(`VC successfully validated`);

    // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    const credentialJSON = signedVc.toJSON();
    return {issuer, credentialJSON};
}

export {createVC};

