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
import { IotaClient as KinesisClient, QueryEventsParams } from "@iota/iota.js/client";
import { getFaucetHost, requestIotaFromFaucetV0 } from "@iota/iota.js/faucet";
import { Ed25519Keypair } from "@iota/iota.js/keypairs/ed25519";
import { Transaction } from "@iota/iota.js/transactions";
import { IOTA_TYPE_ARG } from "@iota/iota.js/utils";
import { IDENTITY_IOTA_PACKAGE_ID, NETWORK_NAME_FAUCET, NETWORK_URL, TEST_GAS_BUDGET } from "../utils_alpha";

async function initializeClients() {
    const kinesis_client = new KinesisClient({ url: NETWORK_URL });

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
    const secret_key = new Uint8Array(VALID_SECP256K1_SECRET_KEY);
    let key_pair = Ed25519Keypair.fromSecretKey(secret_key);
    let pub_key = key_pair.getPublicKey();
    console.log(`Created Ed25519Keypair with PublicKey ${pub_key.toBase64()} and address ${pub_key.toIotaAddress()}`);

    // delete later if not required anymore
    // try to find package beforehand
            // "MoveEventType":"0xac854096fcbfadcdd8cc8e4b6242d1b35607ef5324bfe54ba7a4be69fa6db36d::migration_registry::MigrationRegistryCreated"
            // "Sender": "0xd40005ab355d8342fa6b94e9638a1040483d70430720d28e9b425283d011c0a8"
    const eventsQuery: QueryEventsParams = {
        "query": {
            "MoveEventType":`${IDENTITY_IOTA_PACKAGE_ID}::migration_registry::MigrationRegistryCreated`
        },
        "limit":1,
        "order":"ascending"
    };
    const eventsResult = await kinesis_client.queryEvents(eventsQuery);
    console.dir(eventsResult);

    const identityClientReadOnly = await KinesisIdentityClientReadOnly.createWithPkgId(kinesis_client, IDENTITY_IOTA_PACKAGE_ID);

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

    const balance = await kinesis_client.getBalance({ owner: identityClient.senderAddress() });
    if (balance.totalBalance === "0") {
        throw new Error("Balance is still 0");
    } else {
        console.log(`Received gas from faucet: ${balance.totalBalance} for owner ${identityClient.senderAddress()}`);
    }

    return { kinesis_client, identityClient, key_pair };
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
    kinesis_client: KinesisClient,
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

    const balance = await kinesis_client.getBalance({ owner: identityClient.senderAddress() });
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

    const document1 = new IotaDocument("foobar");
    try {
        // console.log("\n---------------- publishDidDocument ------------------------");
        // not implemented
        // await identityClient.publishDidDocument(document1, BigInt(12345), "dummy signer");
    } catch (ex) {
        console.log(`Test publishDidDocument() - Error: ${(ex as Error).message}`);
    }

    const document2 = new IotaDocument("foobar");
    try {
        // not implemented
        // console.log("\n---------------- publishDidDocumentUpdate ------------------------");
        // await identityClient.publishDidDocumentUpdate(document2, BigInt(12345), "dummy signer");
    } catch (ex) {
        console.log(`Test publishDidDocumentUpdate() - Error: ${(ex as Error).message}`);
    }

    const did4deactivateDidOutput = IotaDID.parse(
        "did:iota:0x0101010101010101010101010101010101010101010101010101010101010101",
    );
    try {
        // not implemented
        // console.log("\n---------------- deactivateDidOutput ------------------------");
        // await identityClient.deactivateDidOutput(did4deactivateDidOutput, BigInt(12345), "dummy signer");
    } catch (ex) {
        console.log(`Test deactivateDidOutput() - Error: ${(ex as Error).message}`);
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

async function testProposals(identityClient: KinesisIdentityClient): Promise<void> {
    console.log(`testProposals disabled after interface updates`);
    // let action: ProposalAction = "Deactivate";
    // console.dir(action);

    // action = { UpdateDocument: new IotaDocument("foobar") };
    // console.dir(action);
    // console.dir(action.UpdateDocument);
    // console.dir(action.UpdateDocument.id());
    // console.dir(action.UpdateDocument.toJSON());

    // let identity = await identityClient
    //     .createIdentity(Uint8Array.from([1, 2, 3]))
    //     .threshold(BigInt(1))
    //     .gasBudget(BigInt(1))
    //     .controllers([
    //         new ControllerAndVotingPower("one", BigInt(1)),
    //         new ControllerAndVotingPower("two", BigInt(2)),
    //     ])
    //     .finish(identityClient, "dummySigner");
    // console.dir(identity);
    // console.dir(identity.isShared());
    // console.dir(identity.proposals());
    // const deactivateProposal = await identity
    //     .deactivateDid()
    //     .expirationEpoch(BigInt(1))
    //     .gasBudget(BigInt(1))
    //     .key("key")
    //     .finish(identityClient, "dummySigner");
    // console.dir(deactivateProposal);

    // // proposals consume the identity instance, so we need a new one
    // identity = await identityClient
    //     .createIdentity(Uint8Array.from([1, 2, 3]))
    //     .threshold(BigInt(1))
    //     .gasBudget(BigInt(1))
    //     .controllers([
    //         new ControllerAndVotingPower("one", BigInt(1)),
    //         new ControllerAndVotingPower("two", BigInt(2)),
    //     ])
    //     .finish(identityClient, "dummySigner");

    // const updateProposal = await identity
    //     .updateDidDocument(new IotaDocument("foobar"))
    //     .expirationEpoch(BigInt(1))
    //     .gasBudget(BigInt(1))
    //     .key("key")
    //     .finish(identityClient, "dummySigner");
    // console.dir(updateProposal);
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

async function testExecuteTransaction(kinesis_client: KinesisClient) {
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
    let coins = await kinesis_client.getCoins({
        owner: address,
        coinType: IOTA_TYPE_ARG,
    });
    const tx = new Transaction();
    const coin_0 = coins.data[0];
    const coin = tx.splitCoins(tx.object(coin_0.coinObjectId), [
        bcs.u64().serialize(TEST_GAS_BUDGET * 2),
    ]);
    tx.transferObjects([coin], address);
    tx.setSenderIfNotSet(address);

    let response = await executeTransaction(
        kinesis_client,
        address,
        publicJwk,
        await tx.build({ client: kinesis_client }),
        signer,
    );
    console.dir(response);
    console.dir(response?.response?.transaction?.data);
}

/** Test API usage */
export async function testApiCall(): Promise<void> {
    const { kinesis_client, identityClient } = await initializeClients();

    try {
        await testIdentityClientReadOnly();
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`identity client binding test failed: ${suffix}`);
    }

    try {
        await testIdentityClient(identityClient, kinesis_client);
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
        await testProposals(identityClient);
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`proposals binding test failed: ${suffix}`);
    }

    try {
        await signerTest();
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`signer binding test failed: ${suffix}`);
    }

    try {
        await testExecuteTransaction(kinesis_client);
    } catch (err) {
        const suffix = err instanceof Error ? `${err.message}; ${err.stack}` : `${err}`;
        console.error(`signer binding test failed: ${suffix}`);
    }

    console.log("done");
}
