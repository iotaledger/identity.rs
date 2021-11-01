// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, Config, VerifiablePresentation } from '@iota/identity-wasm';
import { createVC } from './create_vc';

/**
    This example shows how to create a Verifiable Presentation and validate it.
    A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
    It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

    @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
**/
async function createVP(clientConfig) {
    // Create a default client configuration from the parent config network.
    const config = Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Creates new identities (See "createVC" example)
    const {alice, signedVc} = await createVC(clientConfig);

    // Create a Verifiable Presentation from the Credential - signed by Alice's key
    // TODO: Sign with a challenge
    const unsignedVp = new VerifiablePresentation(alice.doc, signedVc.toJSON())

    const signedVp = alice.doc.signPresentation(unsignedVp, {
        method: "#authentication",
        private: alice.key.private,
    })

    // Check the validation status of the Verifiable Presentation
    const result = await client.checkPresentation(signedVp.toString());

    console.log(`VP verification result: ${result.verified}`);
}

export {createVP};
