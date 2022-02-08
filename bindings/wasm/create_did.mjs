// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {Client, Config, Document, KeyPair, KeyType, Network} from './dist/index.mjs';

console.log(globalThis);

// Generate a new ed25519 public/private key pair.
const key = new KeyPair(KeyType.Ed25519);

// Create a DID Document (an identity) from the generated key pair.
const doc = new Document(key, Network.mainnet().toString());

// Sign the DID Document with the generated key.
doc.signSelf(key, doc.defaultSigningMethod().id);

// Create a default client configuration from the parent config network.
const config = Config.fromNetwork(Network.mainnet());

// Create a client instance to publish messages to the Tangle.
const client = Client.fromConfig(config);

// Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
const receipt = await client.publishDocument(doc);

console.log(receipt);
