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
    RevocationBitmap
} from "./../../node/identity_wasm.js";

/**
 This example shows how to revoke a verifiable credential.
 It demonstrates two methods for revocation. The first uses a revocation bitmap of type `RevocationBitmap2022`,
 while the second method simply removes the verification method (public key) that signed the credential
 from the DID Document of the issuer.

 Note that this example uses the "main" network, if you are writing code against the test network then most function
 calls will need to include information about the network, since this is not automatically inferred from the
 arguments in all cases currently.
 **/
async function revokeVC(storage?: Storage) {
    // ===========================================================================
    // Create a Verifiable Credential.
    // ===========================================================================

    const builder = new AccountBuilder({
        storage
    });

    // Create an identity for the issuer.
    const issuer = await builder.createIdentity();

    // Add a dedicated verification method to the issuer, with which to sign credentials.
    await issuer.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "key-1"
    });

    // Add a RevocationBitmap service to the issuer's DID Document.
    // This allows verifiers to check whether a credential has been revoked.
    const revocationBitmap = new RevocationBitmap();
    await issuer.createService({
        fragment: "my-revocation-service",
        type: RevocationBitmap.type(),
        endpoint: revocationBitmap.toEndpoint()
    });

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: "did:iota:B8DucnzULJ9E8cmaReYoePU2b7UKE9WKxyEVov8tQA7H",
        name: "Alice",
        degree: "Bachelor of Science and Arts",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice.
    // The issuer also chooses a unique `RevocationBitmap` index to be able to revoke it later.
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        credentialStatus: {
            id: issuer.did() + "#my-revocation-service",
            type: RevocationBitmap.type(),
            revocationBitmapIndex: "5"
        },
        issuer: issuer.document().id(),
        credentialSubject: subject
    });

    // Created a signed credential by the issuer.
    const signedVc = await issuer.createSignedCredential(
        "#key-1",
        unsignedVc,
        ProofOptions.default()
    );

    // ===========================================================================
    // Revoke the Verifiable Credential.
    // ===========================================================================

    // Update the RevocationBitmap service in the issuer's DID Document.
    // This revokes the credential's unique index.

    console.log("about to run revocation logic"); 
    //await issuer.unrevokeCredentials("my-revocation-service", 4); 
    //await issuer.revokeCredentials("my-revocation-service", 5);
    await Promise.all([issuer.unrevokeCredentials("my-revocation-service", 4), issuer.revokeCredentials("my-revocation-service", 5)]);

    // Credential verification now fails.
    try {
        console.log("got here");
        CredentialValidator.validate(
            signedVc,
            issuer.document(),
            CredentialValidationOptions.default(),
            FailFast.FirstError
        );
    } catch (e) {
        console.log(`Error during validation: ${e}`);
    }

    // ===========================================================================
    // Alternative revocation of the Verifiable Credential.
    // ===========================================================================

    // By removing the verification method, that signed the credential, from the issuer's DID document,
    // we effectively revoke the credential, as it will no longer be possible to validate the signature.
    await issuer.deleteMethod({
        fragment: "key-1"
    });

    // We expect the verifiable credential to be revoked.
    const resolver = new Resolver();
    try {
        // Resolve the issuer's updated DID Document to ensure the key was revoked successfully.
        const resolvedIssuerDoc = await resolver.resolveCredentialIssuer(
            signedVc
        );
        CredentialValidator.validate(
            signedVc,
            resolvedIssuerDoc,
            CredentialValidationOptions.default(),
            FailFast.FirstError
        );

        // `CredentialValidator.validate` will throw an error, hence this will not be reached.
        console.log("Revocation failed!");
    } catch (e) {
        console.log(`Error during validation: ${e}`);
        console.log(`Credential successfully revoked!`);
    }
}

export { revokeVC };
