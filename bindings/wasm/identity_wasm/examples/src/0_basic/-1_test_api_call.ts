// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    ControllerAndVotingPower,
    convertToAddress,
    IotaDID,
    IotaDocument,
    Jwk,
    JwkMemStore,
    JwsAlgorithm,
    KeyIdMemStore,
    KinesisIdentityClient,
    Multicontroller,
    ProposalAction,
    Storage,
    StorageSigner,
} from "@iota/identity-wasm/node";

import { executeTransaction } from "@iota/identity-wasm/lib/kinesis_client_helpers";
import { bcs } from "@iota/iota.js/bcs";
import { IotaClient as KinesisClient } from "@iota/iota.js/client";
import { getFaucetHost, requestIotaFromFaucetV0, requestIotaFromFaucetV1 } from "@iota/iota.js/faucet";
import { Ed25519Keypair, Ed25519PublicKey } from "@iota/iota.js/keypairs/ed25519";
import { Transaction } from "@iota/iota.js/transactions";
import { IOTA_TYPE_ARG } from "@iota/iota.js/utils";

// this was is the  implemented in `src/kinesis/wasm_identity_client.rs`
// and then imported here as `convertToAddress` from `@iota/identity-wasm/node` (see above)
//
// use fastcrypto::ed25519::Ed25519PublicKey;
// use identity_iota::iota::iota_sdk_abstraction::types::base_types::IotaAddress;
// use identity_iota::iota::sui_name_tbd_error::Error as TbdError;
//
/// TODO: consider importing function from rebased later on, if possible
// pub fn convert_to_address(sender_public_key: &[u8]) -> Result<IotaAddress, TbdError> {
//     let public_key = Ed25519PublicKey::from_bytes(sender_public_key)
//       .map_err(|err| TbdError::InvalidKey(format!("could not parse public key to Ed25519 public key; {err}")))?;
//
//     Ok(IotaAddress::from(&public_key))
//   }
//
//   #[wasm_bindgen(js_name = convertToAddress)]
//   pub fn wasm_convert_to_address(sender_public_key: &[u8]) -> Result<String, JsError> {
//     convert_to_address(sender_public_key)
//       .map(|v| v.to_string())
//       .map_err(|err| JsError::new(&format!("could not derive address from public key; {err}")))
//   }
//
// Currently unclear where to put the this implementation atm. It was a helper for testing performing transactions,
// we might also be able to just drop it due to API restructuring.

export const DEFAULT_GAS_BUDGET = 10000000;
const NETWORK_NAME = "local";
const NETWORK_NAME_FAUCET = "localnet";
const NETWORK_URL = "http://127.0.0.1:9000";
const IDENTITY_IOTA_PACKAGE_ID = "0x7e0ccc737a8def97f37fe9f70267a14bc0fe0871c12f8742fac5e3baf58eb45b";

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

    // test builder and create instance for other tests
    let identityClient = KinesisIdentityClient
        .builder()
        .identityIotaPackageId(IDENTITY_IOTA_PACKAGE_ID)
        .senderPublicKey(pub_key.toRawBytes())
        .senderAddress(pub_key.toIotaAddress())
        .iotaClient(kinesis_client)
        .networkName(NETWORK_NAME)
        .build();

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

async function testIdentityClient(
    identityClient: KinesisIdentityClient,
    kinesis_client: KinesisClient,
    key_pair: Ed25519Keypair,
): Promise<void> {
    console.log("\n-------------- Start testIdentityClient -------------------------------");
    console.log(`chainIdentifier: ${await identityClient.getChainIdentifier()}`);
    console.log(`senderPublicKey: ${identityClient.senderPublicKey()}`);
    console.log(`senderAddress: ${identityClient.senderAddress()}`);
    console.log(`networkName: ${identityClient.networkName()}`);

    try {
        console.log("\n---------------- executeDummyTransaction ------------------------");
        let coins = await kinesis_client.getCoins({
            owner: identityClient.senderAddress(),
            coinType: IOTA_TYPE_ARG,
        });
        const tx = new Transaction();
        const coin_0 = coins.data[0];
        const coin = tx.splitCoins(tx.object(coin_0.coinObjectId), [
            bcs.u64().serialize(DEFAULT_GAS_BUDGET * 2),
        ]);
        tx.transferObjects([coin], identityClient.senderAddress());
        tx.setSenderIfNotSet(key_pair.getPublicKey().toIotaAddress());
        const signatureWithBytes = await tx.sign({ signer: key_pair, client: kinesis_client });

        const response = await identityClient.executeDummyTransaction(
            signatureWithBytes.bytes,
            [signatureWithBytes.signature],
        );
        console.dir(response);

        // The above transaction execution is equivalent to the following snippet using the TS SDK iota client
        const response2 = await kinesis_client.executeTransactionBlock({
            transactionBlock: signatureWithBytes.bytes,
            signature: signatureWithBytes.signature,
        });
        console.log(`TX result: ${response2}`);
    } catch (ex) {
        console.log(`\nTest execute_dummy_transaction() - Error: ${(ex as Error).message}`);
    }

    try {
        console.log("\n---------------- getIdentity ------------------------");
        await identityClient.getIdentity("foobar");
    } catch (ex) {
        console.log(`Test getIdentity() - Error: ${(ex as Error).message}`);
    }

    const did4resolveDid = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        // console.log("\n---------------- resolveDid ------------------------");
        // not implemented
        // await identityClient.resolveDid(did4resolveDid);
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
    let action: ProposalAction = "Deactivate";
    console.dir(action);

    action = { UpdateDocument: new IotaDocument("foobar") };
    console.dir(action);
    console.dir(action.UpdateDocument);
    console.dir(action.UpdateDocument.id());
    console.dir(action.UpdateDocument.toJSON());

    let identity = await identityClient
        .createIdentity(Uint8Array.from([1, 2, 3]))
        .threshold(BigInt(1))
        .gasBudget(BigInt(1))
        .controllers([
            new ControllerAndVotingPower("one", BigInt(1)),
            new ControllerAndVotingPower("two", BigInt(2)),
        ])
        .finish(identityClient, "dummySigner");
    console.dir(identity);
    console.dir(identity.isShared());
    console.dir(identity.proposals());
    const deactivateProposal = await identity
        .deactivateDid()
        .expirationEpoch(BigInt(1))
        .gasBudget(BigInt(1))
        .key("key")
        .finish(identityClient, "dummySigner");
    console.dir(deactivateProposal);

    // proposals consume the identity instance, so we need a new one
    identity = await identityClient
        .createIdentity(Uint8Array.from([1, 2, 3]))
        .threshold(BigInt(1))
        .gasBudget(BigInt(1))
        .controllers([
            new ControllerAndVotingPower("one", BigInt(1)),
            new ControllerAndVotingPower("two", BigInt(2)),
        ])
        .finish(identityClient, "dummySigner");

    const updateProposal = await identity
        .updateDidDocument(new IotaDocument("foobar"))
        .expirationEpoch(BigInt(1))
        .gasBudget(BigInt(1))
        .key("key")
        .finish(identityClient, "dummySigner");
    console.dir(updateProposal);
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
        bcs.u64().serialize(DEFAULT_GAS_BUDGET * 2),
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
    const { kinesis_client, identityClient, key_pair } = await initializeClients();

    type ProgrammableTransaction = ReturnType<typeof bcs.ProgrammableTransaction.parse>;
    let tt: ProgrammableTransaction = bcs.ProgrammableTransaction.parse(new Uint8Array([]));
    console.dir(tt);

    try {
        await testIdentityClient(identityClient, kinesis_client, key_pair);
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
