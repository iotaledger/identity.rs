// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    convertToAddress,
    IotaDID,
    IotaDocument,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    KinesisIdentityClient,
    KinesisIdentityClientReadOnly,
    Multicontroller,
    Storage,
    StorageSigner,
} from "@iota/identity-wasm/node";

import { executeTransaction } from "@iota/iota-interaction-ts/lib/iota_client_helpers";
import { bcs } from "@iota/iota.js/bcs";
import { IotaClient as KinesisClient } from "@iota/iota.js/client";
import { getFaucetHost, requestIotaFromFaucetV0 } from "@iota/iota.js/faucet";
import { Transaction } from "@iota/iota.js/transactions";
import { IOTA_TYPE_ARG } from "@iota/iota.js/utils";
import { IDENTITY_IOTA_PACKAGE_ID, NETWORK_NAME_FAUCET, NETWORK_URL, TEST_GAS_BUDGET } from "../utils_alpha";

async function initializeClients() {
    console.log("---------------- Preparing IdentityClient ------------------------");
    const kinesisClient = new KinesisClient({ url: NETWORK_URL });
    const identityClientReadOnly = await KinesisIdentityClientReadOnly.createWithPkgId(kinesisClient, IDENTITY_IOTA_PACKAGE_ID);

    // create new storage
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());

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

    return { kinesisClient, identityClient };
}


async function testIdentityClientReadOnly() {
    const kinesisClient = new KinesisClient({ url: NETWORK_URL });
    const identityClient = await KinesisIdentityClientReadOnly.createWithPkgId(kinesisClient, IDENTITY_IOTA_PACKAGE_ID);

    console.log("\n-------------- Start testIdentityClientReadOnly -------------------------------");
    console.log(`networkName: ${identityClient.network()}`);
    console.log(`packageId ${identityClient.packageId()}`);
    console.log(`migrationRegistry ${identityClient.migrationRegistryId()}`);
    console.log(`resolveDid ${await identityClient.resolveDid(
        IotaDID.fromAliasId(
        "0x2604b185e0956e5f61549839c2eb5b83274a697ba548ac8d4e474def91a039cc",
        identityClient.network(),
    )
    )}`);
    const identity = await identityClient.getIdentity("0x2604b185e0956e5f61549839c2eb5b83274a697ba548ac8d4e474def91a039cc");
    console.log(`identity.id ${identity.toFullFledged()?.id()}`);
}


async function testIdentityClient(
    identityClient: KinesisIdentityClient,
    kinesisClient: KinesisClient,
): Promise<void> {
    console.log("\n-------------- Start testIdentityClient -------------------------------");
    console.log(`senderPublicKey: ${identityClient.senderPublicKey()}`);
    console.log(`senderAddress: ${identityClient.senderAddress()}`);
    console.log(`networkName: ${identityClient.network()}`);
    console.log(`packageId ${identityClient.packageId()}`);
    console.log(`migrationRegistry ${identityClient.migrationRegistryId()}`);
    console.log(`resolveDid ${await identityClient.resolveDid(
        IotaDID.fromAliasId(
        "0x2604b185e0956e5f61549839c2eb5b83274a697ba548ac8d4e474def91a039cc",
        identityClient.network(),
    )
    )}`);

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

    const newDoc = new IotaDocument(identityClient.network());
    const { output: identity } = await identityClient
        .createIdentity(newDoc)
        .finish()
        .execute(identityClient);
    console.log(`created new identity with id "${identity.id()}"`);

    try {
        console.log("\n---------------- getIdentity ------------------------");
        const identity = await identityClient.getIdentity("0xd9a0f8139076bfbdc245d402c655b4e93cdf5b4184294da2bbbf7ae3d8ec97a4");
        console.dir(identity);
        const onchainIdentity = identity.toFullFledged();
        console.dir(`resolved identities id is ${onchainIdentity?.id()}`);
        console.dir(`resolved identity is shared: ${onchainIdentity?.isShared()}`);

    } catch (ex) {
        console.log(`Test getIdentity() - Error: ${(ex as Error).message}`);
    }

    const did4resolveDid = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        console.log("\n---------------- resolveDid ------------------------");
        // invalid DID
        await identityClient.resolveDid(did4resolveDid);
    } catch (ex) {
        console.log(`Test resolveDid() - Error: ${(ex as Error).message}`);
    }
}

function testMultiController(): void {
    let multiController = new Multicontroller();

    const testCapId = "123";
    console.dir(multiController.controlledValue());
    console.dir(multiController.controllerVotingPower(testCapId));
    console.dir(multiController.hasMember(testCapId));
    console.dir(multiController.intoInner());
    console.dir(multiController.proposals());
    console.dir(multiController.threshold());
}

async function signerTest(): Promise<void> {
    // create new storage
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());

    // generate new key
    let generate = await storage.keyStorage().generate("Ed25519", JwsAlgorithm.EdDSA);
    let publicKeyJwk = generate.jwk().toPublic();
    if (typeof publicKeyJwk === "undefined") {
        throw new Error("failed to derive public JWK from generated JWK");
    }
    let keyId = generate.keyId();
    console.dir({
        keyId,
        publicKeyJwk: publicKeyJwk,
    });

    // create signer from storage
    let signer = new StorageSigner(storage, keyId, publicKeyJwk);
    console.log({ keyIdFromSigner: signer.keyId() });

    // sign test
    let signed = await signer.sign(new Uint8Array([0, 1, 2, 4]));
    console.dir({ signed });
}

async function testExecuteTransaction(kinesisClient: KinesisClient) {
    console.log("---------------- testing executeTransaction ------------------------");

    // create new storage
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());

    // generate new key
    let generate = await storage.keyStorage().generate("Ed25519", JwsAlgorithm.EdDSA);
    let publicKeyJwk = generate.jwk().toPublic();
    if (typeof publicKeyJwk === "undefined") {
        throw new Error("failed to derive public JWK from generated JWK");
    }

    // create signer from storage
    let signer = new StorageSigner(storage, generate.keyId(), publicKeyJwk);
    // get public key as bytes and create address
    let publicJwk = (signer as any).publicKeyRaw();
    let address = convertToAddress(publicJwk);

    await requestIotaFromFaucetV0({
        host: getFaucetHost(NETWORK_NAME_FAUCET),
        recipient: address,
    });

    // try to craft tx with js api
    let coins = await kinesisClient.getCoins({
        owner: address,
        coinType: IOTA_TYPE_ARG,
    });
    const tx = new Transaction();
    const coin0 = coins.data[0];
    const coin = tx.splitCoins(tx.object(coin0.coinObjectId), [
        bcs.u64().serialize(TEST_GAS_BUDGET * BigInt(2)),
    ]);
    tx.transferObjects([coin], address);
    tx.setSenderIfNotSet(address);

    let response = await executeTransaction(
        kinesisClient,
        address,
        publicJwk,
        await tx.build({ client: kinesisClient }),
        signer,
    );
    console.dir(response);
    console.dir(response?.response?.transaction?.data);
}

/** Test API usage */
export async function testApiCall(): Promise<void> {
    const { kinesisClient, identityClient } = await initializeClients();

    try {
        await testIdentityClientReadOnly();
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`identity client binding test failed: ${suffix}`);
    }

    try {
        await testIdentityClient(identityClient, kinesisClient);
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`identity client binding test failed: ${suffix}`);
    }

    try {
        testMultiController();
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`multi controller binding test failed: ${suffix}`);
    }

    try {
        await signerTest();
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`signer binding test failed: ${suffix}`);
    }

    try {
        await testExecuteTransaction(kinesisClient);
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`signer binding test failed: ${suffix}`);
    }

    console.log("done");
}
