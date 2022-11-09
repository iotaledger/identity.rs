// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Bip39 } from "@iota/crypto.js";
import { Client, MnemonicSecretManager } from "@iota/iota-client-wasm/node";
import { IotaIdentityClient, Credential, ProofOptions, CredentialValidator, CredentialValidationOptions, FailFast } from "../../../node";
import { API_ENDPOINT, createDid } from "../util";

export async function createVC() {
    const client = await Client.new({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    const { document: issuerDocument, keypair: keypairIssuer } = await createDid(client, secretManager);

    const { document: aliceDocument } = await createDid(client, secretManager);

     // Create a credential subject indicating the degree earned by Alice, linked to their DID.
     const subject = {
        id: aliceDocument.id(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = new Credential({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuerDocument.id(),
        credentialSubject: subject
    });

    let signedVc = issuerDocument.signData(
        unsignedVc,
        keypairIssuer.private(),
        "#key-1",
        ProofOptions.default()
    )

    // Before sending this credential to the holder the issuer wants to validate that some properties
    // of the credential satisfy their expectations.


    // Validate the credential's signature, the credential's semantic structure,
    // check that the issuance date is not in the future and that the expiration date is not in the past.
    CredentialValidator.validate(
        signedVc,
        issuerDocument,
        CredentialValidationOptions.default(),
        FailFast.AllErrors
    );

    // Since `validate` did not throw any errors we know that the credential was successfully validated.
    console.log(`VC successfully validated`);

    // The issuer is now sure that the credential they are about to issue satisfies their expectations.
    // The credential is then serialized to JSON and transmitted to the holder in a secure manner.
    // Note that the credential is NOT published to the IOTA Tangle. It is sent and stored off-chain.
    const credentialJSON = signedVc.toJSON();
}
