// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDocument, IotaIdentityClient, IotaVerificationMethod, KeyPair, KeyType, MethodScope, X25519 } from '../../../node';
import { IRent, OutputTypes, Bech32Helper, AddressTypes } from '@iota/iota.js';
import { API_ENDPOINT, ensureAddressHasFunds } from '../util';
import { Client, MnemonicSecretManager } from '@iota/iota-client-wasm/node';
import { Bip39 } from '@iota/crypto.js';

/** Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.

Alice and Bob want to communicate securely by encrypting their messages so only they
can read them. They both publish DID Documents with X25519 public keys and use them
to derive a shared secret key for encryption. */
export async function keyExchange() {
  // ==============================
  // Create DIDs for Alice and Bob.
  // ==============================

  // Create a new client to interact with the IOTA ledger.
  const client = new Client({
    primaryNode: API_ENDPOINT,
    localPow: true,
  });
  const didClient = new IotaIdentityClient(client);

  // Generate a random mnemonic for our wallet.
  const secretManager: MnemonicSecretManager = {
    Mnemonic: Bip39.randomMnemonic()
  };

  // Get the Bech32 human-readable part (HRP) of the network.
  const networkName: string = await didClient.getNetworkHrp();

  const addressBech32 = (await client.generateAddresses(secretManager, {
    accountIndex: 0,
    range: {
      start: 0,
      end: 1,
    },
  }))[0];
  const address: AddressTypes = Bech32Helper.addressFromBech32(addressBech32, networkName);

  await ensureAddressHasFunds(client, addressBech32);

  // Get the current byte costs.
  const rentStructure: IRent = await didClient.getRentStructure();

  // Alice creates and publishes their DID Document (see ex0_create_did and 1_update_did examples).
  let aliceDid;
  let aliceX25519;
  {
    // Create a DID Document.
    let aliceDocument: IotaDocument = new IotaDocument(networkName);

    // Insert a new X25519 KeyAgreement verification method.
    const x25519: KeyPair = new KeyPair(KeyType.X25519);
    const method: IotaVerificationMethod = new IotaVerificationMethod(aliceDocument.id(), KeyType.X25519, x25519.public(), "kex-0");
    aliceDocument.insertMethod(method, MethodScope.KeyAgreement());

    // Publish the DID document.
    const aliceOutput: OutputTypes = await didClient.newDidOutput(address, aliceDocument, rentStructure);
    aliceDocument = await didClient.publishDidOutput(secretManager, aliceOutput);

    aliceDid = aliceDocument.id();
    aliceX25519 = x25519;
  }

  // Alice creates and publishes their DID Document (see ex0_create_did and 1_update_did examples).
  let bobDid;
  let bobX25519;
  {
    // Create a DID Document.
    let bobDocument: IotaDocument = new IotaDocument(networkName);

    // Insert a new X25519 KeyAgreement verification method.
    const x25519: KeyPair = new KeyPair(KeyType.X25519);
    const method: IotaVerificationMethod = new IotaVerificationMethod(bobDocument.id(), KeyType.X25519, x25519.public(), "kex-0");
    bobDocument.insertMethod(method, MethodScope.KeyAgreement());

    // Publish the DID document.
    const aliceOutput: OutputTypes = await didClient.newDidOutput(address, bobDocument, rentStructure);
    bobDocument = await didClient.publishDidOutput(secretManager, aliceOutput);

    bobDid = bobDocument.id();
    bobX25519 = x25519;
  }

  // ======================================================================
  // Alice and Bob tell each other their DIDs. They each resolve the
  // DID Document of the other to obtain their X25519 public key.
  // Note that in practice, they would run this code completely separately.
  // ======================================================================

  let aliceSharedSecretKey;
  {
    // Alice: resolves Bob's DID Document and extracts their public key.
    const bobDocument: IotaDocument = await didClient.resolveDid(bobDid);
    const bobMethod: IotaVerificationMethod = bobDocument.resolveMethod("kex-0", MethodScope.KeyAgreement())!;
    const bobPublicKey: Uint8Array = bobMethod.data().tryDecode();
    // Compute the shared secret.
    aliceSharedSecretKey = X25519.keyExchange(aliceX25519.private(), bobPublicKey);
  }

  let bobSharedSecretKey;
  {
    // Bob: resolves Alice's DID Document and extracts their public key.
    const aliceDocument: IotaDocument = await didClient.resolveDid(aliceDid);
    const aliceMethod: IotaVerificationMethod = aliceDocument.resolveMethod("kex-0", MethodScope.KeyAgreement())!;
    const alicePublicKey: Uint8Array = aliceMethod.data().tryDecode();
    // Compute the shared secret.
    bobSharedSecretKey = X25519.keyExchange(bobX25519.private(), alicePublicKey);
  }

  // Both shared secret keys computed separately by Alice and Bob will match
  // and can then be used to establish encrypted communications.
  if (!isArrayEqual(aliceSharedSecretKey, bobSharedSecretKey)) throw new Error("shared secret keys do not match!");

  console.log(`Diffie-Hellman key exchange successful!`);
}

function isArrayEqual(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}
