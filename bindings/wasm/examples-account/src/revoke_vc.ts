// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    Credential,
    CredentialValidationOptions,
    CredentialValidator,
    FailFast,
    Resolver,
    Storage,
    MethodContent,
    ProofOptions,
    RevocationBitmap,
} from './../../node/identity_wasm.js';


/**
 This example shows how to revoke a verifiable credential.
 The Verifiable Credential is revoked by actually removing a verification method (public key) from the DID Document of the Issuer.
 As such, the Verifiable Credential can no longer be validated.
 This would invalidate every Verifiable Credential signed with the same public key, therefore the issuer would have to sign every VC with a different key.

 Note that this example uses the "main" network, if you are writing code against the test network then most function
 calls will need to include information about the network, since this is not automatically inferred from the
 arguments in all cases currently.
 **/
async function revokeVC(storage?: Storage) {
    // ===========================================================================
    // Create a Verifiable Credential.
    // ===========================================================================

    const builder = new AccountBuilder({
        storage,
    });

    // Create an identity for the issuer.
    const issuer = await builder.createIdentity();

    // Add a dedicated verification method to the issuer, with which to sign credentials.
    await issuer.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "key-1"
    })

    // Add the EmbeddedRevocationService for allowing verfiers to check the credential status.
    const revocationBitmap = new RevocationBitmap;
    await issuer.createService({
        fragment: "my-revocation-service",
        type: "RevocationBitmap2022",
        endpoint: "data:," + revocationBitmap.serializeCompressedB64()
    })

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: "did:iota:B8DucnzULJ9E8cmaReYoePU2b7UKE9WKxyEVov8tQA7H",
        name: "Alice",
        degree: "Bachelor of Science and Arts",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice 
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        credentialStatus: {
            id: issuer.did()+"#my-revocation-service",
            type_: "RevocationBitmap2022",
            revocationListIndex: "5"
        },
        issuer: issuer.document().id(),
        credentialSubject: subject,
    });

    // Created a signed credential by the issuer.
    const signedVc = await issuer.createSignedCredential(
        "#key-1",
        unsignedVc,
        ProofOptions.default(),
    );

    // ===========================================================================
    // Revoke the Verifiable Credential.
    // ===========================================================================

    // Update the service for checking the credential status
    // When verifiers look for the index corresponding to the credential, it will be set to revoked.
    await issuer.revokeCredentials("my-revocation-service", new Uint32Array([5]))
    try {
        CredentialValidator.validate(
            signedVc,
            issuer.document(),
            CredentialValidationOptions.default(),
            FailFast.FirstError
        );
    } catch (e) {
        console.log(`Error During validation: ${e}`)
    }

    await issuer.deleteMethod({
        fragment: "key-1"
    })

    // Check the verifiable credential.
    const resolver = new Resolver();
    try {
        // Resolve the issuer's updated DID Document to ensure the key was revoked successfully.
        const resolvedIssuerDoc = await resolver.resolveCredentialIssuer(signedVc);
        CredentialValidator.validate(
            signedVc,
            resolvedIssuerDoc,
            CredentialValidationOptions.default(),
            FailFast.FirstError
        );

        // `CredentialValidator.validate` will throw an error, hence this will not be reached.
        console.log("Revocation failed!");
    } catch (e) {
        console.log(`Error During validation: ${e}`)
        console.log(`Credential successfully revoked!`);
    }
}

export { revokeVC };
