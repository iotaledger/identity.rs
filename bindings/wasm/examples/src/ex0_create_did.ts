// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    KeyPair,
    KeyType,
    MethodScope,
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    IotaVerificationMethod
} from '../../node';
import { Bech32Helper, IAliasOutput } from '@iota/iota.js';
import { Bip39 } from "@iota/crypto.js";
import fetch from "node-fetch";
import { Client, MnemonicSecretManager, SecretManager } from "@iota/iota-client-wasm/node";

const API_ENDPOINT = "https://api.testnet.shimmer.network/";
const FAUCET = "https://faucet.testnet.shimmer.network/api/enqueue";

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function createIdentity(): Promise<{
    didClient: IotaIdentityClient,
    secretManager: SecretManager,
    walletAddressBech32: string,
    did: IotaDID
}> {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkHrp: string = await didClient.getNetworkHrp();

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        Mnemonic: Bip39.randomMnemonic()
    };
    const walletAddressBech32 = (await client.generateAddresses(secretManager, {
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
    }))[0];
    console.log("Wallet address Bech32:", walletAddressBech32);

    // Request funds for the wallet, if needed - only works on development networks.
    await ensureAddressHasFunds(client, walletAddressBech32);

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    const document = new IotaDocument(networkHrp);

    // Insert a new Ed25519 verification method in the DID document.
    let keypair = new KeyPair(KeyType.Ed25519);
    let method = new IotaVerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-1");
    document.insertMethod(method, MethodScope.VerificationMethod());

    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    const address = Bech32Helper.addressFromBech32(walletAddressBech32, networkHrp);
    const aliasOutput: IAliasOutput = await didClient.newDidOutput(address, document);
    console.log("Alias Output:", JSON.stringify(aliasOutput, null, 2));

    // Publish the Alias Output and get the published DID document.
    const published = await didClient.publishDidOutput(secretManager, aliasOutput);
    console.log("Published DID document:", JSON.stringify(published, null, 2));

    return {
        didClient, secretManager,
        walletAddressBech32,
        did: published.id()
    };
}

/** Request funds from the testnet faucet API, if needed, and wait for them to show in the wallet. */
async function ensureAddressHasFunds(client: Client, addressBech32: string) {
    let balance = await getAddressBalance(client, addressBech32);
    if (balance > 0) {
        return;
    }

    await requestFundsFromFaucet(addressBech32);

    for (let i = 0; i < 9; i++) {
        // Wait for the funds to reflect.
        await new Promise(f => setTimeout(f, 5000));

        let balance = await getAddressBalance(client, addressBech32);
        if (balance > 0) {
            break;
        }
    }
}

/** Returns the balance of the given Bech32-encoded address. */
async function getAddressBalance(client: Client, addressBech32: string): Promise<number> {
    // TODO: use the `addresses/ed25519/<addressHex>` API to get the balance?
    const outputIds = await client.basicOutputIds([
        { address: addressBech32 },
        { hasExpiration: false },
        { hasTimelock: false },
        { hasStorageDepositReturn: false }
    ]);
    const outputs = await client.getOutputs(outputIds);

    let totalAmount = 0;
    for (const output of outputs) {
        totalAmount += Number(output.output.amount);
    }

    return totalAmount;
}

/** Request tokens from the testnet faucet API. */
async function requestFundsFromFaucet(addressBech32: string) {
    const requestObj = JSON.stringify({ address: addressBech32 });
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
