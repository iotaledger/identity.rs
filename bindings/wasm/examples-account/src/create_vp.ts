// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    Credential,
    CredentialValidationOptions,
    Duration,
    FailFast,
    Presentation,
    PresentationValidationOptions,
    ProofOptions,
    Resolver,
    SubjectHolderRelationship,
    Timestamp,
    VerifierOptions,
    Storage,
    MethodContent,
    Network
} from './../../node/identity_wasm.js';

/**
 This example shows how to create a Verifiable Presentation and validate it.
 A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
 It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.
 **/
async function createVP(storage?: Storage) {
    // ===========================================================================
    // Step 1: Create identities for the issuer and the holder.
    // ===========================================================================

    // Create Account builder.
    const builder = new AccountBuilder({
        storage,
    });

    // Create an identity for the issuer.
    const issuer = await builder.createIdentity();

    // Add a dedicated verification method to the issuer, with which to sign credentials.
    await issuer.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "issuerKey"
    })

    // Create an identity for the holder, in this case also the subject.
    const alice = await builder.createIdentity();

    // Add verification method to the holder.
    await alice.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "aliceKey"
    })

    // ===========================================================================
    // Step 2: Issuer creates and signs a Verifiable Credential.
    // ===========================================================================

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: alice.document().id(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.document().id(),
        credentialSubject: subject
    });

    // Created a signed credential by the issuer.
    const signedVc = await issuer.createSignedCredential(
        "#issuerKey",
        unsignedVc,
        ProofOptions.default(),
    );

    // ===========================================================================
    // Step 3: Issuer sends the Verifiable Credential to the holder.
    // ===========================================================================

    // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    const signedVcJson = signedVc.toJSON();
    console.log(`Credential JSON >`, JSON.stringify(signedVcJson, null, 2));


    // ===========================================================================
    // Step 4: Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
    // ===========================================================================

    // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
    const challenge = "475a7984-1bb5-4c4c-a56f-822bccd46440";

    // The verifier and holder also agree that the signature should have an expiry date
    // 10 minutes from now.
    const expires = Timestamp.nowUTC().checkedAdd(Duration.minutes(10));


    // ===========================================================================
    // Step 5: Holder creates a verifiable presentation from the issued credential for the verifier to validate.
    // ===========================================================================

    // Deserialize the credential.
    const receivedVc = Credential.fromJSON(signedVcJson);

    // Create a Verifiable Presentation from the Credential
    const unsignedVp = new Presentation({
        holder: alice.did(),
        verifiableCredential: receivedVc
    })

    // Sign the verifiable presentation using the holder's verification method
    // and include the requested challenge and expiry timestamp.
    const signedVp = await alice.createSignedPresentation(
        "#aliceKey",
        unsignedVp,
        new ProofOptions({
            challenge: challenge,
            expires
        })
    );

    // ===========================================================================
    // Step 6: Holder sends a verifiable presentation to the verifier.
    // ===========================================================================

    // Convert the Verifiable Presentation to JSON to send it to the verifier.
    const signedVpJSON = signedVp.toJSON();

    // ===========================================================================
    // Step 7: Verifier receives the Verifiable Presentation and verifies it.
    // ===========================================================================

    // Deserialize the presentation from the holder.
    const presentation = Presentation.fromJSON(signedVpJSON);

    // The verifier wants the following requirements to be satisfied:
    // - Signature verification (including checking the requested challenge to mitigate replay attacks)
    // - Presentation validation must fail if credentials expiring within the next 10 hours are encountered
    // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
    // - The issuance date must not be in the future.

    // Declare that the challenge must match our expectation:
    const presentationVerifierOptions = new VerifierOptions({
        challenge: "475a7984-1bb5-4c4c-a56f-822bccd46440",
        allowExpired: false,
    });

    // Declare that any credential contained in the presentation are not allowed to expire within the next 10 hours:
    const earliestExpiryDate = Timestamp.nowUTC().checkedAdd(Duration.hours(10));
    const credentialValidationOptions = new CredentialValidationOptions({
        earliestExpiryDate: earliestExpiryDate,
    });

    // Declare that the presentation holder's DID must match the subject ID on all credentials in the presentation.
    const subjectHolderRelationship = SubjectHolderRelationship.AlwaysSubject;

    const presentationValidationOptions = new PresentationValidationOptions({
        sharedValidationOptions: credentialValidationOptions,
        presentationVerifierOptions: presentationVerifierOptions,
        subjectHolderRelationship: subjectHolderRelationship,
    });

    // In order to validate presentations and credentials one needs to resolve the DID Documents of
    // the presentation holder and of credential issuers. This is something the `Resolver` can help with.
    const resolver = new Resolver();

    // Validate the presentation and all the credentials included in it according to the validation options
    // TODO: uncomment when ported to Stardust.
    // await resolver.verifyPresentation(
    //     presentation,
    //     presentationValidationOptions,
    //     FailFast.FirstError
    // );

    // Since no errors were thrown by `verifyPresentation` we know that the validation was successful.
    console.log(`VP successfully validated`);

    // Note that the `verifyPresentation` method we called automatically resolved all DID Documents that are necessary to validate the presentation.
    // It is also possible to supply extra arguments to avoid some resolutions if one already has up-to-date resolved documents of
    // either the holder or issuers (see the method's documentation).
}

export { createVP };
