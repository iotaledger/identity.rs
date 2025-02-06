// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaDocument,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    KinesisIdentityClient,
    KinesisIdentityClientReadOnly,
    MethodScope,
    Storage,
    StorageSigner,
} from "@iota/identity-wasm/node";
import { IotaClient as KinesisClient } from "@iota/iota.js/client";
import { getFaucetHost, requestIotaFromFaucetV0 } from "@iota/iota.js/faucet";

export const IDENTITY_IOTA_PACKAGE_ID =
  process.env.IDENTITY_IOTA_PACKAGE_ID || "0xac854096fcbfadcdd8cc8e4b6242d1b35607ef5324bfe54ba7a4be69fa6db36d";
export const NETWORK_NAME_FAUCET = "localnet";
export const NETWORK_URL =
  process.env.NETWORK_URL || "http://127.0.0.1:9000";
export const TEST_GAS_BUDGET = BigInt(50_000_000);

export function getMemstorage(): Storage {
    return new Storage(new JwkMemStore(), new KeyIdMemStore());
}

export async function createDocumentForNetwork(storage: Storage, network: string): Promise<[IotaDocument, string]> {
    // Create a new DID document with a placeholder DID.
    const unpublished = new IotaDocument(network);

    const verificationMethodFragment = await unpublished.generateMethod(
        storage,
        JwkMemStore.ed25519KeyType(),
        JwsAlgorithm.EdDSA,
        "#key-1",
        MethodScope.VerificationMethod(),
    );

    return [unpublished, verificationMethodFragment];
}

export async function getClientAndCreateAccount(storage: Storage): Promise<KinesisIdentityClient> {
    if (!IDENTITY_IOTA_PACKAGE_ID) {
        throw new Error(`IDENTITY_IOTA_PACKAGE_ID env variable must be provided to run the examples`);
    }

    const kinesisClient = new KinesisClient({ url: NETWORK_URL });

    const identityClientReadOnly = await KinesisIdentityClientReadOnly.createWithPkgId(
        kinesisClient, IDENTITY_IOTA_PACKAGE_ID);

    // generate new key
    let generate = await storage.keyStorage().generate("Ed25519", JwsAlgorithm.EdDSA);
    let publicKeyJwk = generate.jwk().toPublic();
    if (typeof publicKeyJwk === "undefined") {
        throw new Error("failed to derive public JWK from generated JWK");
    }
    let keyId = generate.keyId();

    // create signer from storage
    let signer = new StorageSigner(storage, keyId, publicKeyJwk);
    const identityClient = await KinesisIdentityClient.create(identityClientReadOnly, signer);

    await requestIotaFromFaucetV0({
        host: getFaucetHost(NETWORK_NAME_FAUCET),
        recipient: identityClient.senderAddress(),
    });

    const balance = await kinesisClient.getBalance({ owner: identityClient.senderAddress() });
    if (balance.totalBalance === "0") {
        throw new Error("Balance is still 0");
    } else {
        console.log(`Received gas from faucet: ${balance.totalBalance} for owner ${identityClient.senderAddress()}`);
    }

    return identityClient;
}

export async function createDidDocument(
    identityClient: KinesisIdentityClient,
    unpublished: IotaDocument,
): Promise<IotaDocument> {
    let tx = identityClient
      .publishDidDocument(unpublished);
    let txOutput = await tx.execute(identityClient);
  
    return txOutput.output;
  }
  