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
} from '@iota/identity-wasm/node';

/**
 * Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.
 **/
async function keyExchange() {
    // Create a client instance to publish messages to the configured Tangle network.
    const client = new Client();

    // Alice creates and publishes their DID Document (see create_did and manipulate_did examples).
    let aliceDID;
    let aliceX25519;
    {
        // Create a DID Document.
        const keypair = new KeyPair(KeyType.Ed25519);
        const document = new Document(keypair);

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
        const document = new Document(keypair);

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
    // to obtain their X25519 public key. Note that in practice, they would run this code completely
    // separately.

    let aliceSharedSecretKey;
    {
        // Alice: resolves Bob's DID Document and extracts their public key.
        const bobDocument = await client.resolve(bobDID);
        const bobMethod = bobDocument.intoDocument().resolveMethod("kex-0", MethodScope.KeyAgreement());

        if(!bobMethod) {
            throw new Error('Method not found');
        }

        const bobPublicKey = bobMethod.data().tryDecode();
        // Compute the shared secret.
        aliceSharedSecretKey = X25519.keyExchange(aliceX25519.private(), bobPublicKey);
    }

    let bobSharedSecretKey;
    {
        // Bob: resolves Alice's DID Document and extracts their public key.
        const aliceDocument = await client.resolve(aliceDID);
        const aliceMethod = aliceDocument.intoDocument().resolveMethod("kex-0", MethodScope.KeyAgreement());

        if(!aliceMethod) {
            throw new Error('Method not found');
        }

        const alicePublicKey = aliceMethod.data().tryDecode();
        // Compute the shared secret.
        bobSharedSecretKey = X25519.keyExchange(bobX25519.private(), alicePublicKey);
    }

    // Both shared secret keys computed separately by Alice and Bob will match
    // and can then be used to establish encrypted communications.
    if(!isArrayEqual(aliceSharedSecretKey, bobSharedSecretKey)) throw new Error("shared secret keys do not match!");
    console.log(`Diffie-Hellman key exchange successful!`);
}

function isArrayEqual(a: Uint8Array, b: Uint8Array) {
    if(a.length !== b.length) return false;
    for(let i = 0; i < a.length; i++) {
        if(a[i] !== b[i]) return false;
    }
    return true;
}

export {keyExchange};
