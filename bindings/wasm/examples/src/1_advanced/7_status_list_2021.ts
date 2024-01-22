// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    FailFast,
    IJwsVerifier,
    IotaIdentityClient,
    Jwk,
    JwkMemStore,
    JwsAlgorithm,
    JwsSignatureOptions,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    KeyIdMemStore,
    StatusCheck,
    StatusList2021,
    StatusList2021Credential,
    StatusList2021CredentialBuilder,
    StatusList2021Entry,
    StatusPurpose,
    Storage,
    verifyEd25519,
} from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

export async function statusList2021() {
    // ===========================================================================
    // Create a Verifiable Credential.
    // ===========================================================================

    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });

    // Generate a random mnemonic for the issuer.
    const issuerSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Create an identity for the issuer with one verification method `key-1`.
    const issuerStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: issuerDocument, fragment: issuerFragment } = await createDid(
        client,
        issuerSecretManager,
        issuerStorage,
    );

    // Generate a random mnemonic for Alice.
    const aliceSecretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Create an identity for the holder, in this case also the subject.
    const aliceStorage: Storage = new Storage(
        new JwkMemStore(),
        new KeyIdMemStore(),
    );
    let { document: aliceDocument } = await createDid(
        client,
        aliceSecretManager,
        aliceStorage,
    );

    // Create a new empty status list. No credential is revoked yet.
    const statusList = new StatusList2021();

    // Create a status list credential so that the status list can be stored anywhere.
    // In this example the credential will fictitiously be made available at `http://example.com/credential/status`
    // (actually it will stay in memory).
    const statusListCredential = new StatusList2021CredentialBuilder(statusList)
        .purpose(StatusPurpose.Revocation)
        .subjectId("http://example.com/credential/status")
        .issuer(issuerDocument.id().toString())
        .build();
    const statusListCredentialJSON = statusListCredential.toJSON();

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: aliceDocument.id(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0",
    };

    // Create an unsigned `UniversityDegree` credential for Alice.
    // The issuer also chooses a unique `StatusList2021` index to be able to revoke it later.
    const CREDENTIAL_INDEX = 5;
    const status = new StatusList2021Entry(statusListCredential.id(), statusListCredential.purpose(), CREDENTIAL_INDEX)
        .toStatus();
    const credential = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        credentialStatus: status,
        issuer: issuerDocument.id(),
        credentialSubject: subject,
    });

    // Create signed JWT credential.
    const credentialJwt = await issuerDocument.createCredentialJwt(
        issuerStorage,
        issuerFragment,
        credential,
        new JwsSignatureOptions(),
    );
    console.log(`Credential JWT > ${credentialJwt.toString()}`);

    // Validate the credential using the issuer's DID Document.
    const validationOptions = new JwtCredentialValidationOptions({ status: StatusCheck.SkipUnsupported });
    // The validator has no way of retrieving the status list to check for the
    // revocation of the credential. Let's skip that pass and perform the operation manually.
    let jwtCredentialValidator = new JwtCredentialValidator(new Ed25519JwsVerifier());
    jwtCredentialValidator.validate(
        credentialJwt,
        issuerDocument,
        validationOptions,
        FailFast.FirstError,
    );

    // ===========================================================================
    // Revocation of the Verifiable Credential.
    // ===========================================================================

    // The issuer retrieves the status list credential.
    const refetchedStatusListCredential = new StatusList2021Credential(new Credential(statusListCredentialJSON as any));

    // Update the status list credential.
    // This revokes the credential's unique index.
    refetchedStatusListCredential.setCredentialStatus(credential, CREDENTIAL_INDEX, true);

    // Credential verification now fails.
    try {
        jwtCredentialValidator.validate(
            credentialJwt,
            issuerDocument,
            validationOptions,
            FailFast.FirstError,
        );
        /// Since the credential has been revoked, this validation step will throw an error.
        JwtCredentialValidator.checkStatusWithStatusList2021(
            credential,
            refetchedStatusListCredential,
            StatusCheck.Strict,
        );
        console.log("Revocation Failed!");
    } catch (e) {
        /// The credential has been revoked.
        console.log(`Error during validation: ${e}`);
    }
}

// A custom JWS Verifier capabale of verifying EdDSA signatures with curve Ed25519.
class Ed25519JwsVerifier implements IJwsVerifier {
    verify(alg: JwsAlgorithm, signingInput: Uint8Array, decodedSignature: Uint8Array, publicKey: Jwk) {
        switch (alg) {
            case JwsAlgorithm.EdDSA:
                // This verifies that the curve is Ed25519 so we don't need to check ourselves.
                return verifyEd25519(alg, signingInput, decodedSignature, publicKey);
            default:
                throw new Error(`unsupported jws algorithm ${alg}`);
        }
    }
}
