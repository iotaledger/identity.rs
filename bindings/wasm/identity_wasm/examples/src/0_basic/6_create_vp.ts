// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoreDID,
    Credential,
    Duration,
    EdDSAJwsVerifier,
    FailFast,
    IdentityClientReadOnly,
    IotaDocument,
    JwsSignatureOptions,
    JwsVerificationOptions,
    Jwt,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    JwtPresentationOptions,
    JwtPresentationValidationOptions,
    JwtPresentationValidator,
    Presentation,
    Resolver,
    SubjectHolderRelationship,
    Timestamp,
} from "@iota/identity-wasm/node";
import { IotaClient } from "@iota/iota-sdk/client";
import { createDocumentForNetwork, getFundedClient, getMemstorage, IOTA_IDENTITY_PKG_ID, NETWORK_URL } from "../util";

/**
 * This example shows how to create a Verifiable Presentation and validate it.
 * A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
 * It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.
 */
export async function createVP() {
    // ===========================================================================
    // Step 1: Create identities for the issuer and the holder.
    // ===========================================================================

    // create new client to connect to IOTA network
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();

    // create issuer account, create identity, and publish DID document for it
    const issuerStorage = getMemstorage();
    const issuerClient = await getFundedClient(issuerStorage);
    const [unpublishedIssuerDocument, issuerFragment] = await createDocumentForNetwork(issuerStorage, network);
    const { output: issuerIdentity } = await issuerClient
        .createIdentity(unpublishedIssuerDocument)
        .finish()
        .execute(issuerClient);
    const issuerDocument = issuerIdentity.didDocument()!;

    // create holder account, create identity, and publish DID document for it
    const aliceStorage = getMemstorage();
    const aliceClient = await getFundedClient(aliceStorage);
    const [unpublishedAliceDocument, aliceFragment] = await createDocumentForNetwork(aliceStorage, network);
    const { output: aliceIdentity } = await aliceClient
        .createIdentity(unpublishedAliceDocument)
        .finish()
        .execute(aliceClient);
    const aliceDocument = aliceIdentity.didDocument()!;

    // ===========================================================================
    // Step 2: Issuer creates and signs a Verifiable Credential.
    // ===========================================================================

    const subject = {
        id: aliceDocument.id(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0",
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuerDocument.id(),
        credentialSubject: subject,
    });

    const credentialJwt = await issuerDocument.createCredentialJwt(
        issuerStorage,
        issuerFragment,
        unsignedVc,
        new JwsSignatureOptions(),
    );

    const res = new JwtCredentialValidator(new EdDSAJwsVerifier()).validate(
        credentialJwt,
        issuerDocument,
        new JwtCredentialValidationOptions(),
        FailFast.FirstError,
    );
    console.log("credentialjwt validation", res.intoCredential());

    // ===========================================================================
    // Step 3: Issuer sends the Verifiable Credential to the holder.
    // ===========================================================================

    // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    console.log(`Sending credential (as JWT) to the holder`, unsignedVc.toJSON());

    // ===========================================================================
    // Step 4: Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
    // ===========================================================================

    // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
    const nonce = "475a7984-1bb5-4c4c-a56f-822bccd46440";

    // The verifier and holder also agree that the signature should have an expiry date
    // 10 minutes from now.
    const expires = Timestamp.nowUTC().checkedAdd(Duration.minutes(10));

    // ===========================================================================
    // Step 5: Holder creates a verifiable presentation from the issued credential for the verifier to validate.
    // ===========================================================================

    // Create a Verifiable Presentation from the Credential
    const unsignedVp = new Presentation({
        holder: aliceDocument.id(),
        verifiableCredential: [credentialJwt],
    });

    // Create a JWT verifiable presentation using the holder's verification method
    // and include the requested challenge and expiry timestamp.
    const presentationJwt = await aliceDocument.createPresentationJwt(
        aliceStorage,
        aliceFragment,
        unsignedVp,
        new JwsSignatureOptions({ nonce }),
        new JwtPresentationOptions({ expirationDate: expires }),
    );

    // ===========================================================================
    // Step 6: Holder sends a verifiable presentation to the verifier.
    // ===========================================================================
    console.log(
        `Sending presentation (as JWT) to the verifier`,
        unsignedVp.toJSON(),
    );

    // ===========================================================================
    // Step 7: Verifier receives the Verifiable Presentation and verifies it.
    // ===========================================================================

    // The verifier wants the following requirements to be satisfied:
    // - JWT verification of the presentation (including checking the requested challenge to mitigate replay attacks)
    // - JWT verification of the credentials.
    // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
    // - The issuance date must not be in the future.

    const jwtPresentationValidationOptions = new JwtPresentationValidationOptions(
        {
            presentationVerifierOptions: new JwsVerificationOptions({ nonce }),
        },
    );

    const resolver = new Resolver<IotaDocument>({
        client: await IdentityClientReadOnly.createWithPkgId(iotaClient, IOTA_IDENTITY_PKG_ID),
    });
    // Resolve the presentation holder.
    const presentationHolderDID: CoreDID = JwtPresentationValidator.extractHolder(presentationJwt);
    const resolvedHolder = await resolver.resolve(
        presentationHolderDID.toString(),
    );

    // Validate presentation. Note that this doesn't validate the included credentials.
    let decodedPresentation = new JwtPresentationValidator(new EdDSAJwsVerifier()).validate(
        presentationJwt,
        resolvedHolder,
        jwtPresentationValidationOptions,
    );

    // Validate the credentials in the presentation.
    let credentialValidator = new JwtCredentialValidator(new EdDSAJwsVerifier());
    let validationOptions = new JwtCredentialValidationOptions({
        subjectHolderRelationship: [
            presentationHolderDID.toString(),
            SubjectHolderRelationship.AlwaysSubject,
        ],
    });

    let jwtCredentials: Jwt[] = decodedPresentation
        .presentation()
        .verifiableCredential()
        .map((credential) => {
            const jwt = credential.tryIntoJwt();
            if (!jwt) {
                throw new Error("expected a JWT credential");
            } else {
                return jwt;
            }
        });

    // Concurrently resolve the issuers' documents.
    let issuers: string[] = [];
    for (let jwtCredential of jwtCredentials) {
        let issuer = JwtCredentialValidator.extractIssuerFromJwt(jwtCredential);
        issuers.push(issuer.toString());
    }
    let resolvedIssuers = await resolver.resolveMultiple(issuers);

    // Validate the credentials in the presentation.
    for (let i = 0; i < jwtCredentials.length; i++) {
        credentialValidator.validate(
            jwtCredentials[i],
            resolvedIssuers[i],
            validationOptions,
            FailFast.FirstError,
        );
    }

    // Since no errors were thrown we know that the validation was successful.
    console.log(`VP successfully validated`);
}
