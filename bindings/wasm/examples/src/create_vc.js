// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Config, Credential, CredentialValidator, SignatureOptions, CredentialValidationOptions, Resolver, FailFast} from '@iota/identity-wasm';
import {createIdentity} from './create_did';
import {manipulateIdentity} from './manipulate_did';

/**
 This example shows how to create a Verifiable Credential and validate it.
 In this example, alice takes the role of the subject, while we also have an issuer.
 The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
 This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with whoever they please.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function createVC(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates new identities (See "create_did" and "manipulate_did" examples)
    const alice = await createIdentity(clientConfig);
    const issuer = await manipulateIdentity(clientConfig);

    // Prepare a credential subject indicating the degree earned by Alice
    let credentialSubject = {
        id: alice.doc.id.toString(),
        name: "Alice",
        degreeName: "Bachelor of Science and Arts",
        degreeType: "BachelorDegree",
        GPA: "4.0"
    };

    // Create an unsigned `UniversityDegree` credential for Alice
    const unsignedVc = Credential.extend({
        id: "https://example.edu/credentials/3732",
        type: "UniversityDegreeCredential",
        issuer: issuer.doc.id.toString(),
        credentialSubject,
    });

    // Sign the credential with the Issuer's newKey
    const signedVc = issuer.doc.signCredential(unsignedVc, {
        method: issuer.doc.id.toString() + "#newKey",
        public: issuer.newKey.public,
        private: issuer.newKey.private,
    }, SignatureOptions.default());

    // Before passing this credential on the issuer wants to validate that some properties
    // of the credential satisfy their expectations.
    // In order to validate a credential the issuer's DID Document needs to be resolved.
    // Since the issuer wants to issue and verify several credentials without publishing updates to their DID Document
    // the issuer decides to resolve their DID Document up front now so they can re-use it.

    const resolver = await new Resolver(); 
    const issuerDoc = await resolver.resolve(issuer.doc.id.toString()); 

    // Validate the credential's signature, the credential's semantic structure, 
    // check that the issuance date is not in the future and that the expiration date is not in the past. We use `FailFast.No`
    // to ensure that if validation fails then the error message will contain information about every unsuccessful validation.  
    const result = CredentialValidator.validate(
        signedVc,
        CredentialValidationOptions.default(),
        issuerDoc,
        FailFast.No
    );

    console.log(`VC validated: ${result}`);

    // The issuer is now sure that the credential they are about to issue satisfies their expectations
    // hence the credential is now serialized to JSON before passing it to the subject in a secure manner.
    // This means that the credential is NOT published to the tangle where it can be accessed by anyone.

    const credentialJSON = signedVc.toString();
    return {alice, issuer, credentialJSON};
}

export {createVC};
