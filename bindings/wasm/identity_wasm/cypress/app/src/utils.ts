// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IdentityClient,
    IdentityClientReadOnly,
    IotaDocument,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    MethodScope,
    Storage,
    StorageSigner,
} from "@iota/identity-wasm/web";
import { IotaClient } from "@iota/iota-sdk/client";
import { getFaucetHost, requestIotaFromFaucetV0 } from "@iota/iota-sdk/faucet";

export const IOTA_IDENTITY_PKG_ID = "0x222741bbdff74b42df48a7b4733185e9b24becb8ccfbafe8eac864ab4e4cc555";
export const NETWORK_NAME_FAUCET = "testnet";
export const NETWORK_URL = "https://api.testnet.iota.cafe";
export const TEST_GAS_BUDGET = BigInt(10_000_000);

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

export async function getFundedClient(storage: Storage): Promise<IdentityClient> {
    if (!IOTA_IDENTITY_PKG_ID) {
        throw new Error(`IOTA_IDENTITY_PKG_ID env variable must be provided to run the examples`);
    }

    const iotaClient = new IotaClient({ url: NETWORK_URL });

    const identityClientReadOnly = await IdentityClientReadOnly.createWithPkgId(
        iotaClient,
        IOTA_IDENTITY_PKG_ID,
    );

    // generate new key
    let generate = await storage.keyStorage().generate("Ed25519", JwsAlgorithm.EdDSA);

    let publicKeyJwk = generate.jwk().toPublic();
    if (typeof publicKeyJwk === "undefined") {
        throw new Error("failed to derive public JWK from generated JWK");
    }
    let keyId = generate.keyId();

    // create signer from storage
    let signer = new StorageSigner(storage, keyId, publicKeyJwk);
    const identityClient = await IdentityClient.create(identityClientReadOnly, signer);

    await requestIotaFromFaucetV0({
        host: getFaucetHost(NETWORK_NAME_FAUCET),
        recipient: identityClient.senderAddress(),
    });

    const balance = await iotaClient.getBalance({ owner: identityClient.senderAddress() });
    if (balance.totalBalance === "0") {
        throw new Error("Balance is still 0");
    } else {
        console.log(`Received gas from faucet: ${balance.totalBalance} for owner ${identityClient.senderAddress()}`);
    }

    return identityClient;
}

export async function createDidDocument(
    identityClient: IdentityClient,
    unpublished: IotaDocument,
): Promise<IotaDocument> {
    let txOutput = await identityClient
        .publishDidDocument(unpublished)
        .execute(identityClient);

    return txOutput.output;
}
