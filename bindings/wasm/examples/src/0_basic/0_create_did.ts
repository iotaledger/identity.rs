// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager, SecretManager } from "@iota/client-wasm/node";
import { Bip39 } from "@iota/crypto.js";
import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    KeyPair,
    KeyType,
    MethodScope,
    VerificationMethod,
} from "@iota/identity-wasm/node";
import { Bech32Helper, IAliasOutput } from "@iota/iota.js";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function createIdentity(): Promise<{
    didClient: IotaIdentityClient;
    secretManager: SecretManager;
    walletAddressBech32: string;
    did: IotaDID;
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
        mnemonic: Bip39.randomMnemonic(),
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
    let method = new VerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-1");
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
        didClient,
        secretManager,
        walletAddressBech32,
        did: published.id(),
    };
}
