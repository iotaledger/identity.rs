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
} from "@iota/identity-wasm/node";
import { IotaClient as KinesisClient } from "@iota/iota.js/client";
import { getFaucetHost, requestIotaFromFaucetV0 } from "@iota/iota.js/faucet";
import { Ed25519Keypair } from "@iota/iota.js/keypairs/ed25519";

export const TEST_GAS_BUDGET = 50_000_000;

const IOTA_LOCAL_NETWORK_URL = "http://127.0.0.1:9000";
const NETWORK_NAME = "local";
const NETWORK_NAME_FAUCET = "localnet"

const {
    API_ENDPOINT,
    IDENTITY_IOTA_PACKAGE_ID,
} = process.env;

export function getMemstorage(): Storage {
    return new Storage(new JwkMemStore(), new KeyIdMemStore());
}

export async function getClientAndCreateAccount(storage: Storage): Promise<KinesisIdentityClient> {
    let api_endpoint = API_ENDPOINT || IOTA_LOCAL_NETWORK_URL;
    if (!IDENTITY_IOTA_PACKAGE_ID) {
        throw new Error(`IDENTITY_IOTA_PACKAGE_ID env variable must be provided to run the examples`);
    }

    const kinesisClient = new KinesisClient({ url: api_endpoint });

    // generate new key
    // TODO: make random key
    console.log("---------------- Preparing IdentityClient ------------------------");
    const VALID_SECP256K1_SECRET_KEY = [
        59,
        148,
        11,
        85,
        134,
        130,
        61,
        253,
        2,
        174,
        59,
        70,
        27,
        180,
        51,
        107,
        94,
        203,
        174,
        253,
        102,
        39,
        170,
        146,
        46,
        252,
        4,
        143,
        236,
        12,
        136,
        28,
    ];
    const secretKey = new Uint8Array(VALID_SECP256K1_SECRET_KEY);
    let keyPair = Ed25519Keypair.fromSecretKey(secretKey);
    let pubKey = keyPair.getPublicKey();
    console.log(`Created Ed25519Keypair with PublicKey ${pubKey.toBase64()} and address ${pubKey.toIotaAddress()}`);

    // test builder and create instance for other tests
    // let identityClient = KinesisIdentityClient
    //     .builder()
    //     .identityIotaPackageId(IDENTITY_IOTA_PACKAGE_ID)
    //     .senderPublicKey(pubKey.toRawBytes())
    //     .senderAddress(pubKey.toIotaAddress())
    //     .iotaClient(kinesisClient)
    //     .networkName(NETWORK_NAME)
    //     .build();
    const identityClientReadOnly = KinesisIdentityClientReadOnly.create(kinesisClient);
    console.dir(identityClientReadOnly);
    const identityClient = null as any;

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
    identity_client: KinesisIdentityClient,
    storage: Storage,
): Promise<[IotaDocument, String]> {
    // Create a new DID document with a placeholder DID.
    const unpublished = new IotaDocument(identity_client.network());
    
    throw new Error('createDidDocument not fully implemented');

    // // Insert a new Ed25519 verification method in the DID document.
    // await unpublished.generateMethod(
    //     storage,
    //     JwkMemStore.ed25519KeyType(),
    //     JwsAlgorithm.EdDSA,
    //     "#key-1",
    //     MethodScope.VerificationMethod(),
    // );

    // let verification_method_fragment = await unpublished
    //     .generate_method(
    //         storage,
    //         JwkMemStore.ed25519KeyType(),
    //         JwsAlgorithm.EdDSA,
    //         "#key-1", // TODO: `None` in Rust? oO
    //         MethodScope.VerificationMethod(),
    //     );

    // // TODO: add publish
    // // (Dummy interface)
    // // await identityClient.publishDidDocument(document, BigInt(12345), "dummy signer");
    // // (Rust)
    // // let document = identity_client
    // //   .publish_did_document(unpublished)
    // //   .execute_with_gas(TEST_GAS_BUDGET, identity_client)
    // //   .await?
    // //   .output;
  
    // return [document, verification_method_fragment];
  }
  