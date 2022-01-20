// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IdentitySetup, AccountBuilder, KeyPair, KeyType, DID, Credential, VerifierOptions
} from './../../node/identity_wasm.js';

/**
 * This example shows a basic introduction on how to create a basic DID Document and upload it to the Tangle
 * using the Account.
 */
async function signing() {

    // ===========================================================================
    // Create Identity - Similar to create_did example
    // ===========================================================================

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    let builder = new AccountBuilder({
    });
    let account = await builder.createIdentity(new IdentitySetup());

    // ===========================================================================
    // Signing Example
    // ===========================================================================

    // Add a new Ed25519 Verification Method to the identity
    await account.createMethod({
        fragment: "key_1"
    })

    // Create a subject DID for the recipient of a `UniversityDegree` credential.
    let keyPair: KeyPair = new KeyPair(KeyType.Ed25519);
    let subjectDid = new DID(keyPair);

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: subjectDid.toString(),
        name: "Alice",
        degree: {
            type: "BachelorDegree",
            name: "Bachelor of Science and Arts"
        }
    };

    // Issue an unsigned Credential...
    const unsignedVc = Credential.extend({
        issuer: account.did().toString(),
        type: "UniversityDegreeCredential",
        credentialSubject,
    });

    // ...and sign the Credential with the previously created Verification Method
    // Note: Different methods are available for different data types
    // use the Method `createSignedData` to sign arbitrary data.
    let signedVc = await account.createSignedCredential("key_1", unsignedVc, {});

    console.log("[Example] Local Credential", signedVc);

    // Fetch the DID Document from the Tangle
    //
    // This is an optional step to ensure DID Document consistency.
    let resolved = await account.resolveIdentity();

    // Retrieve the DID from the newly created identity.
    let did = account.did();

    console.log("[Example] DID = ", did.toString());

    // Ensure the resolved DID Document can verify the credential signature
    let verified = resolved.intoDocument().verifyData(signedVc, VerifierOptions.default());

    console.log("[Example] Credential Verified = ", verified);
}

export { signing };
