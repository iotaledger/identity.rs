// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, DIDMessageEncoding, Document, ExplorerUrl, KeyPair, KeyType, Network} from '@iota/identity-wasm';

/**
 This example shows how a DID document can be created on a private tangle.
 It can be run together with a local hornet node.
 Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
 for setup instructions.
 **/
async function privateTangle(restURL, networkName) {
    // This name needs to match the id of the network or part of it.
    // Since the id of the one-click private tangle is `private-tangle`
    // but we can only use 6 characters, we use just `tangle`.
    const network = Network.tryFromName(networkName || "tangle");

    // Optionally point to a locally-deployed Tangle explorer.
    const explorer = ExplorerUrl.parse("http://127.0.0.1:8082/");

    // Create a client instance with a custom configuration to publish messages to our private Tangle.
    const client = await Client.fromConfig({
        network: network,
        // This URL points to the REST API of the locally running hornet node.
        primaryNode: {url: restURL || "http://127.0.0.1:14265/"},
        // Use DIDMessageEncoding.Json instead to publish plaintext messages to the Tangle for debugging.
        encoding: DIDMessageEncoding.JsonBrotli,
    });

    // Generate a new ed25519 public/private key pair.
    const key = new KeyPair(KeyType.Ed25519);

    // Create a DID with the network set explicitly.
    // This will result in a DID prefixed by `did:iota:tangle`.
    const doc = new Document(key, network.name());

    // Sign the DID Document with the generated key.
    doc.signSelf(key, doc.defaultSigningMethod().id());

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc);

    // Make sure the DID can be resolved on the private tangle
    const resolved = await client.resolve(doc.id());

    console.log(`Published the DID document to the private tangle:`);
    console.log(resolved);
    console.log(`Explore the DID Document: ${explorer.resolverUrl(doc.id())}`);

    // Return the results.
    return {key, resolved, receipt};
}

export {privateTangle};
