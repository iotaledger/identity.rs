// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {StardustDocument} from '../../node';

import {
    Bech32Helper,
    ED25519_ADDRESS_TYPE,
    Ed25519Address,
    Ed25519Seed,
    IAliasOutput,
    SingleNodeClient,
} from '@iota/iota.js';

import {StardustIdentityClient} from "./stardust_identity_client";
import {Converter} from "@iota/util.js";
import {NeonPowProvider} from "@iota/pow-neon.js";
import {Bip32Path} from "@iota/crypto.js";
import {randomBytes} from "node:crypto";
import fetch from "node-fetch";

process.env["NODE_TLS_REJECT_UNAUTHORIZED"] = "0";
const EXPLORER = "https://explorer.alphanet.iotaledger.net/alphanet";
const API_ENDPOINT = "https://api.alphanet.iotaledger.net/";
const FAUCET = "https://faucet.alphanet.iotaledger.net/api/enqueue";

// In this example we set up a hot wallet, fund it with tokens from the faucet and let it mint an NFT to our address.
async function run() {
    // LocalPoW is extremely slow and only runs in 1 thread...
    // const client = new SingleNodeClient(API_ENDPOINT, {powProvider: new LocalPowProvider()});
    // Neon localPoW is blazingly fast, but you need rust toolchain to build
    const client = new SingleNodeClient(API_ENDPOINT, {powProvider: new NeonPowProvider()});
    const didClient = new StardustIdentityClient(client);
    const protocolInfo = await client.protocolInfo();
    const network: string = protocolInfo.bech32Hrp;

    // Now it's time to set up an account for this demo which we are going to use to mint nft and send it to the target address.
    console.log("Sender Address:");
    const [walletAddressHex, walletAddressBech32, walletKeyPair] = await setUpHotWallet(network, true);
    console.log("\tAddress Ed25519", walletAddressHex);
    console.log("\tAddress Bech32", walletAddressBech32);

    const document = new StardustDocument(network);
    const aliasOutput: IAliasOutput = await didClient.newDidOutput(ED25519_ADDRESS_TYPE, walletAddressHex, document);
    console.log("AliasOutput", JSON.stringify(aliasOutput, null, 4));


    const published = await didClient.publishDidOutput(walletKeyPair, aliasOutput);
    console.log("Published DID document: ", published);
}

run()
    .then(() => console.log("Done"))
    .catch(err => console.error(err));

async function setUpHotWallet(hrp: string, fund: boolean = false) {
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
    const walletAddressHex = Converter.bytesToHex(walletAddress, true);

    let walletAddressBech32 = Bech32Helper.toBech32(ED25519_ADDRESS_TYPE, walletAddress, hrp);

    // We also top up the address by asking funds from the faucet.
    if (fund) {
        await requestFundsFromFaucet(walletAddressBech32);
    }

    return [walletAddressHex, walletAddressBech32, walletKeyPair] as const;
}

// Requests frunds from the faucet via API
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
