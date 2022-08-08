// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    AccountBuilder,
    Credential,
    DID,
    ExplorerUrl,
    KeyPair,
    KeyType,
    MethodContent,
    ProofOptions,
    Storage,
    VerifierOptions
} from './../../node';

/**
 * This example demonstrates how to issue and sign Verifiable Credentials using the account.
 */
async function signing(storage?: Storage) {

    // ===========================================================================
    // Create Identity - Similar to create_did example
    // ===========================================================================

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({
        storage,
    });
    let account = await builder.createIdentity();

    // ===========================================================================
    // Signing Example
    // ===========================================================================

    // Add a new Ed25519 Verification Method to the identity for signing issued verifiable credentials.
    await account.createMethod({
        content: MethodContent.GenerateEd25519(),
        fragment: "key_1"
    })

    // Prepare a credential subject indicating the degree earned by Alice, linked to their DID.
    const subject = {
        id: "did:iota:3TT7QsmESw1dcboV2oVTAuJbbxaVYAThexZFct2z5Q2d",
        name: "Alice",
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts"
        }
    };

    // Issue an unsigned Credential...
    const unsignedVc = new Credential({
        issuer: account.did(),
        type: "UniversityDegreeCredential",
        credentialSubject: subject,
    });

    // ...and sign the Credential with the previously created Verification Method.
    // Note: Different methods are available for different data types,
    // use the Method `createSignedData` to sign arbitrary data.
    let signedVc = await account.createSignedCredential("key_1", unsignedVc, ProofOptions.default());

    console.log("[Example] Local Credential", signedVc);

    // Fetch the DID Document from the Tangle.
    //
    // This is an optional step to ensure DID Document consistency.
    let resolved = await account.resolveIdentity();

    // Retrieve the DID from the newly created identity.
    let did = account.did().toString();

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));

    // Ensure the resolved DID Document can verify the credential signature.
    let verified = resolved.intoDocument().verifyData(signedVc, VerifierOptions.default());

    console.log("[Example] Credential Verified = ", verified);
}

export {signing};
