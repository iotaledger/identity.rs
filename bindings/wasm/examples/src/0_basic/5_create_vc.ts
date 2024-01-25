// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Credential,
    EdDSAJwsVerifier,
    FailFast,
    JwkMemStore,
    JwsSignatureOptions,
    JwtCredentialValidationOptions,
    JwtCredentialValidator,
    KeyIdMemStore,
    Storage,
} from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

/**
 * This example shows how to create a Verifiable Credential and validate it.
 * In this example, Alice takes the role of the subject, while we also have an issuer.
 * The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
 * This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whomever they please.
 */
export async function createVC() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Create an identity for the issuer with one verification method `key-1`.
    const issuerStorage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document: issuerDocument, fragment: issuerFragment } = await createDid(
        client,
        secretManager,
        issuerStorage,
    );

    // Create an identity for the holder, in this case also the subject.
    const aliceStorage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document: aliceDocument } = await createDid(
        client,
        secretManager,
        aliceStorage,
    );

    // Create a credential subject indicating the degree earned by Alice, linked to their DID.
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

    // Create signed JWT credential.
    const credentialJwt = await issuerDocument.createCredentialJwt(
        issuerStorage,
        issuerFragment,
        unsignedVc,
        new JwsSignatureOptions(),
    );
    console.log(`Credential JWT > ${credentialJwt.toString()}`);

    // Before sending this credential to the holder the issuer wants to validate that some properties
    // of the credential satisfy their expectations.

    // Validate the credential's signature, the credential's semantic structure,
    // check that the issuance date is not in the future and that the expiration date is not in the past.
    // Note that the validation returns an object containing the decoded credential.
    const decoded_credential = new JwtCredentialValidator(new EdDSAJwsVerifier()).validate(
        credentialJwt,
        issuerDocument,
        new JwtCredentialValidationOptions(),
        FailFast.FirstError,
    );

    // Since `validate` did not throw any errors we know that the credential was successfully validated.
    console.log(`VC successfully validated`);

    // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    console.log(`Issued credential: ${JSON.stringify(decoded_credential.intoCredential(), null, 2)}`);
}
