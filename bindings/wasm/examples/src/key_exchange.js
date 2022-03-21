// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    Document,
    KeyPair,
    KeyType,
    MethodScope,
    VerificationMethod,
    X25519,
} from '@iota/identity-wasm';

/**
 Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.

 @param {{network: Network, explorer: ExplorerUrl}} clientConfig
 **/
async function keyExchange(clientConfig) {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = await Client.fromConfig({
        network: clientConfig.network
    });

    // Alice creates and publishes their DID Document (see create_did and manipulate_did examples).
    let aliceDID;
    let aliceX25519;
    {
        // Create a DID Document.
        const keypair = new KeyPair(KeyType.Ed25519);
        const document = new Document(keypair, clientConfig.network);

        // Insert a new X25519 KeyAgreement verification method.
        let x25519 = new KeyPair(KeyType.X25519);
        let method = new VerificationMethod(document.id(), KeyType.X25519, x25519.public(), "kex-0");
        document.insertMethod(method, MethodScope.KeyAgreement());

        // Publish the DID Document.
        document.signSelf(keypair, document.defaultSigningMethod().id());
        await client.publishDocument(document);

        aliceDID = document.id();
        aliceX25519 = x25519;
    }

    // Bob creates and publishes their DID Document (see create_did and manipulate_did examples).
    let bobDID;
    let bobX25519;
    {
        // Create a DID Document.
        const keypair = new KeyPair(KeyType.Ed25519);
        const document = new Document(keypair, clientConfig.network);

        // Insert a new X25519 KeyAgreement verification method.
        let x25519 = new KeyPair(KeyType.X25519);
        let method = new VerificationMethod(document.id(), KeyType.X25519, x25519.public(), "kex-0");
        document.insertMethod(method, MethodScope.KeyAgreement());

        // Publish the DID Document.
        document.signSelf(keypair, document.defaultSigningMethod().id());
        await client.publishDocument(document);

        bobDID = document.id();
        bobX25519 = x25519;
    }

    // Alice and Bob tell each other their DIDs. They each resolve the DID Document of the other
    // to obtain their X25519 public key. Note that in practise, they would run this code completely
    // separately.

    let aliceSharedSecretKey;
    {
        // Alice: resolves Bob's DID Document and extracts their public key.
        const bobDocument = await client.resolve(bobDID);
        const bobMethod = bobDocument.intoDocument().resolveMethod("kex-0", MethodScope.KeyAgreement());
        const bobPublicKey = bobMethod.data().tryDecode();
        // Compute the shared secret.
        aliceSharedSecretKey = X25519.keyExchange(aliceX25519.private(), bobPublicKey);
    }

    let bobSharedSecretKey;
    {
        // Bob: resolves Alice's DID Document and extracts their public key.
        const aliceDocument = await client.resolve(aliceDID);
        const aliceMethod = aliceDocument.intoDocument().resolveMethod("kex-0", MethodScope.KeyAgreement());
        const alicePublicKey = aliceMethod.data().tryDecode();
        // Compute the shared secret.
        bobSharedSecretKey = X25519.keyExchange(bobX25519.private(), alicePublicKey);
    }

    // Both shared secret keys computed separately by Alice and Bob will match,
    // and they can then use it to establish encrypted communications.
    if(aliceSharedSecretKey !== bobSharedSecretKey) throw new Error("shared secret keys do not match!");
    console.log(`Diffie-Hellman key exchange successful!`);
}

export {keyExchange};
