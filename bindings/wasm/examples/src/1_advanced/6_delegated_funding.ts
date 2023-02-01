// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager, SecretManager } from "@iota/client-wasm/node";
import { Bip39 } from "@iota/crypto.js";
import {
    IotaDID,
    IotaDocument,
    IotaIdentityClient,
    IotaVerificationMethod,
    KeyPair,
    KeyType,
    MethodScope,
} from "@iota/identity-wasm/node";
import {
    ALIAS_OUTPUT_TYPE,
    Bech32Helper,
    IAliasOutput,
    ITransactionPayload,
    IUTXOInput,
    PayloadTypes,
    TRANSACTION_ESSENCE_TYPE,
    TRANSACTION_PAYLOAD_TYPE,
    TransactionHelper,
} from "@iota/iota.js";
import { Converter } from "@iota/util.js";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

export async function delegatedFunding(): Promise<void> {
    // Setup

    const { funder, controller } = await setup();

    // Acquire funds for Funder from faucet.

    // Create an initial identity / alias output funded by the funder and controlled by the controller.
    // We do this so that we have an alias output to modify and show what we actually want to show:
    // delegated funding when updating an alias output. Delegated funding when creating an alias output
    // is a special case of this example.
    const { outputId, aliasOutput } = await createIdentity(funder.client, funder.secretManager, controller.address);

    // TODO:
    // controller.updateAlias(...)
}

class Controller {
    secretManager: SecretManager;
    address: string;

    constructor(secretManager: SecretManager, address: string) {
        this.secretManager = secretManager;
        this.address = address;
    }

    // Update the DID document in the alias.
    async updateAlias(
        outputId: string,
        aliasOutput: IAliasOutput,
        tokenSupply: number,
    ): Promise<void> {
        // TODO: Find a way to convert the outputId into a `IUTXOInput`.
        // const input: IUTXOInput = outputId;
    }
}

class Funder {
    secretManager: SecretManager;
    client: Client;
    address: string;

    constructor(secretManager: SecretManager, client: Client, address: string) {
        this.secretManager = secretManager;
        this.client = client;
        this.address = address;
    }
}

async function setup(): Promise<{ controller: Controller; funder: Funder }> {
    const controllerSecretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    const funderSecretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });

    const controllerAddressBech32 = (await client.generateAddresses(controllerSecretManager, {
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
    }))[0];

    const funderAddressBech32 = (await client.generateAddresses(controllerSecretManager, {
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
    }))[0];

    await ensureAddressHasFunds(client, funderAddressBech32);

    const controller = new Controller(controllerSecretManager, controllerAddressBech32);
    const funder = new Funder(funderSecretManager, client, funderAddressBech32);

    return {
        controller,
        funder,
    };
}

async function createIdentity(
    client: Client,
    secretManager: SecretManager,
    addressBech32: string,
): Promise<{
    outputId: string;
    aliasOutput: IAliasOutput;
}> {
    const didClient = new IotaIdentityClient(client);

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkHrp: string = await didClient.getNetworkHrp();

    // Create a new DID document with a placeholder DID.
    // The DID will be derived from the Alias Id of the Alias Output after publishing.
    const document = new IotaDocument(networkHrp);

    // Insert a new Ed25519 verification method in the DID document.
    let keypair = new KeyPair(KeyType.Ed25519);
    let method = new IotaVerificationMethod(document.id(), keypair.type(), keypair.public(), "#key-1");
    document.insertMethod(method, MethodScope.VerificationMethod());

    // Construct an Alias Output containing the DID document, with the wallet address
    // set as both the state controller and governor.
    const address = Bech32Helper.addressFromBech32(addressBech32, networkHrp);
    const aliasOutput: IAliasOutput = await didClient.newDidOutput(address, document);
    console.log("Alias Output:", JSON.stringify(aliasOutput, null, 2));

    const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [aliasOutput] });
    await client.retryUntilIncluded(blockId);

    // Extract the output ID of the Alias from the published block.
    // Non-null assertion is safe because we published a block with a payload.
    const outputId = alias_output_id(block.payload!);

    return {
        outputId,
        aliasOutput,
    };
}

function alias_output_id(payload: PayloadTypes): string {
    if (payload.type === TRANSACTION_PAYLOAD_TYPE) {
        const txPayload: ITransactionPayload = payload;
        const txHash = Converter.bytesToHex(TransactionHelper.getTransactionPayloadHash(txPayload), true);

        if (txPayload.essence.type === TRANSACTION_ESSENCE_TYPE) {
            const outputs = txPayload.essence.outputs;
            for (let index in txPayload.essence.outputs) {
                if (outputs[index].type === ALIAS_OUTPUT_TYPE) {
                    const outputId: string = TransactionHelper.outputIdFromTransactionData(txHash, parseInt(index));
                    return TransactionHelper.resolveIdFromOutputId(outputId);
                }
            }
            throw new Error("no Alias Output in transaction essence");
        } else {
            throw new Error("expected transaction essence");
        }
    } else {
        throw new Error("expected transaction payload");
    }
}
