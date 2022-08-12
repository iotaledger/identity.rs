// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {KeyPair, KeyType, MethodScope, StardustDID, StardustDocument, StardustVerificationMethod} from '../../node';

import {
    Bech32Helper,
    ED25519_ADDRESS_TYPE,
    Ed25519Address,
    Ed25519Seed,
    IAliasOutput,
    IEd25519Address,
    IKeyPair,
} from '@iota/types';

import {StardustIdentityClient} from "./stardust_identity_client";
import {Converter} from "@iota/types/util.js";
import {Bip32Path, Bip39} from "@iota/types/crypto.js";
import {randomBytes} from "node:crypto";
import fetch from "node-fetch";
import {Client, SecretManager} from "@iota/client";

const EXPLORER = "https://explorer.alphanet.iotaledger.net/alphanet";
const API_ENDPOINT = "https://api.alphanet.iotaledger.net/";
const FAUCET = "https://faucet.alphanet.iotaledger.net/api/enqueue";

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function createIdentity(): Promise<{
    didClient: StardustIdentityClient,
    secretManager: SecretManager,
    // walletAddress: Ed25519Address,
    // did: StardustDID
}> {
    // Allow self-signed TLS certificates when running in Node.js.
    // WARNING: this is generally insecure and should not be done in production.
    // if (typeof process !== 'undefined' && process.release.name === 'node') {
    //     process.env["NODE_TLS_REJECT_UNAUTHORIZED"] = "0";
    // }

    const client = new Client({
        primaryNode: API_ENDPOINT,
    });
    const didClient = new StardustIdentityClient(client);

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkHrp: string = await didClient.getNetworkHrp();


    // Configure Stronghold to hold our wallet keys.
    const secretManager = {
        Stronghold: {
            // WARNING: do not do this, store and access your password securely in production.
            password: 'some strong random password',
            snapshotPath: 'client.stronghold',
        },
    };

    // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
    // The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
    await client.storeMnemonic(
        secretManager,
        Bip39.randomMnemonic(),
    );

    const address = await client.generateAddresses(secretManager, {
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
    });
    console.log("Address:", address);

    // Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
    // The mnemonic cannot be retrieved from the Stronghold file, so make a backup in a secure place!

    // Create a new wallet and request funds for it from the faucet (only works on test networks).
    // console.log("Sender Address:");
    // const [walletAddress, walletKeyPair] = await setUpHotWallet(networkHrp, true);
    // const walletAddressHex: string = Converter.bytesToHex(walletAddress.toAddress(), true);
    // const walletAddressBech32: string = Bech32Helper.toBech32(ED25519_ADDRESS_TYPE, walletAddress.toAddress(), networkHrp);
    // console.log("\tAddress Ed25519", walletAddressHex);
    // console.log("\tAddress Bech32", walletAddressBech32);
    //
    // // Create a new DID document with a placeholder DID.
    // // The DID will be derived from the Alias Id of the Alias Output after publishing.
    // const document = new StardustDocument(networkHrp);
    //
    // // Insert a new Ed25519 verification method in the DID document.
    // let keypair = new KeyPair(KeyType.Ed25519);
    // let method = new StardustVerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-1");
    // document.insertMethod(method, MethodScope.VerificationMethod());
    //
    // // Construct an Alias Output containing the DID document, with the wallet address
    // // set as both the state controller and governor.
    // const address: IEd25519Address = {
    //     type: ED25519_ADDRESS_TYPE,
    //     pubKeyHash: walletAddressHex
    // };
    // const aliasOutput: IAliasOutput = await didClient.newDidOutput(address, document);
    // console.log("Alias Output:", JSON.stringify(aliasOutput, null, 2));
    //
    // // Publish the Alias Output and get the published DID document.
    // const published = await didClient.publishDidOutput(secretManager, aliasOutput);
    // console.log("Published DID document:", JSON.stringify(published, null, 2));

    return {didClient, secretManager,
        // walletAddress,
        // did: published.id()
    };
}

/** Generate a new Ed25519 wallet address and optionally fund it from the faucet API. */
async function setUpHotWallet(networkHrp: string, fund: boolean = false): Promise<[Ed25519Address, IKeyPair]> {
    // Generate a random seed
    const walletEd25519Seed = new Ed25519Seed(randomBytes(32));

    // For Shimmer we use Coin Type 4219
    const path = new Bip32Path("m/44'/4219'/0'/0'/0'");

    // Construct wallet from seed
    const walletSeed = walletEd25519Seed.generateSeedFromPath(path);
    let walletKeyPair = walletSeed.keyPair();

    console.log("\tSeed", Converter.bytesToHex(walletSeed.toBytes()));

    // Get the wallet address, which is the Blake2b-256 digest of the public key.
    const walletEd25519Address = new Ed25519Address(walletKeyPair.publicKey);
    const walletAddress = walletEd25519Address.toAddress();
    let walletAddressBech32 = Bech32Helper.toBech32(ED25519_ADDRESS_TYPE, walletAddress, networkHrp);

    // We also top up the address by asking funds from the faucet.
    if (fund) {
        await requestFundsFromFaucet(walletAddressBech32);
    }

    return [walletEd25519Address, walletKeyPair];
}

/** Request tokens from the faucet API. */
async function requestFundsFromFaucet(addressBech32: string) {
    const requestObj = JSON.stringify({address: addressBech32});
    let errorMessage, data;
    try {
        const response = await fetch(FAUCET, {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: requestObj,
        });
        if (response.status === 202) {
            errorMessage = "OK";
        } else if (response.status === 429) {
            errorMessage = "too many requests, please try again later.";
        } else {
            data = await response.json();
            // @ts-ignore
            errorMessage = data.error.message;
        }
    } catch (error) {
        errorMessage = error;
    }

    if (errorMessage != "OK") {
        throw new Error(`failed to get funds from faucet: ${errorMessage}`);
    }
}
