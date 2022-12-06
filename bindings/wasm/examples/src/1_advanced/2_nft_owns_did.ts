// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, MnemonicSecretManager } from "@iota/client-wasm/node";
import { Bip39 } from "@iota/crypto.js";
import { IotaDocument, IotaIdentityClient } from "@iota/identity-wasm/node";
import {
    ADDRESS_UNLOCK_CONDITION_TYPE,
    AddressTypes,
    Bech32Helper,
    IAliasOutput,
    INftOutput,
    IOutputResponse,
    IRent,
    IStateControllerAddressUnlockCondition,
    ITransactionPayload,
    NFT_ADDRESS_TYPE,
    NFT_OUTPUT_TYPE,
    OutputTypes,
    PayloadTypes,
    STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE,
    TRANSACTION_ESSENCE_TYPE,
    TRANSACTION_PAYLOAD_TYPE,
    TransactionHelper,
} from "@iota/iota.js";
import { Converter } from "@iota/util.js";
import { API_ENDPOINT, ensureAddressHasFunds } from "../util";

/** Demonstrates how an identity can be owned by NFTs,
and how observers can verify that relationship.

For this example, we consider the case where a car's NFT owns
the DID of the car, so that transferring the NFT also transfers DID ownership. */
export async function nftOwnsDid() {
    // =============================
    // Create the car's NFT and DID.
    // =============================

    // Create a new Client to interact with the IOTA ledger.
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Bip39.randomMnemonic(),
    };

    // Get the current byte costs.
    const rentStructure: IRent = await didClient.getRentStructure();

    // Get the Bech32 human-readable part (HRP) of the network.
    const networkName: string = await didClient.getNetworkHrp();

    // Create a new address that will own the NFT.
    const addressBech32 = (await client.generateAddresses(secretManager, {
        accountIndex: 0,
        range: {
            start: 0,
            end: 1,
        },
    }))[0];
    const address = Bech32Helper.addressFromBech32(addressBech32, networkName);

    // Get funds for testing from the faucet.
    await ensureAddressHasFunds(client, addressBech32);

    // Create the car NFT with an Ed25519 address as the unlock condition.
    let carNft: INftOutput = await client.buildNftOutput({
        nftId: "0x0000000000000000000000000000000000000000000000000000000000000000",
        unlockConditions: [
            {
                // The NFT will initially be owned by the Ed25519 address.
                type: ADDRESS_UNLOCK_CONDITION_TYPE,
                address,
            },
        ],
    });

    // Set the appropriate storage deposit.
    carNft.amount = TransactionHelper.getStorageDeposit(carNft, rentStructure).toString();

    // Publish the NFT.
    const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [carNft] });
    await client.retryUntilIncluded(blockId);

    // Extract the identifier of the NFT from the published block.
    // Non-null assertion is safe because we published a block with a payload.
    var carNftId: string = nft_output_id(block.payload!);

    // Create the address of the NFT.
    const nftAddress: AddressTypes = {
        type: NFT_ADDRESS_TYPE,
        nftId: carNftId,
    };

    // Construct a DID document for the car.
    var carDocument: IotaDocument = new IotaDocument(networkName);

    // Create a new DID for the car that is owned by the car NFT.
    var carDidAliasOutput: IAliasOutput = await didClient.newDidOutput(nftAddress, carDocument, rentStructure);

    // Publish the car DID.
    carDocument = await didClient.publishDidOutput(secretManager, carDidAliasOutput);

    // ============================================
    // Determine the car's NFT given the car's DID.
    // ============================================

    // Resolve the Alias Output of the DID.
    carDidAliasOutput = await didClient.resolveDidOutput(carDocument.id());

    // Extract the NFT Id from the state controller unlock condition.
    const stateControllerUnlockCondition: IStateControllerAddressUnlockCondition = carDidAliasOutput.unlockConditions
        .find(feature =>
            feature.type === STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE
        ) as IStateControllerAddressUnlockCondition;
    if (stateControllerUnlockCondition.address.type === NFT_ADDRESS_TYPE) {
        carNftId = stateControllerUnlockCondition.address.nftId;
    } else {
        throw new Error("expected nft address unlock condition");
    }

    // Fetch the NFT Output of the car.
    const nftOutputId: string = await client.nftOutputId(carNftId);
    const outputResponse: IOutputResponse = await client.getOutput(nftOutputId);
    const output: OutputTypes = outputResponse.output;

    if (output.type === NFT_OUTPUT_TYPE) {
        carNft = output;
    } else {
        throw new Error("expected nft output type");
    }

    console.log("The car's DID is:", JSON.stringify(carDocument, null, 2));
    console.log("The car's NFT is:", JSON.stringify(carNft, null, 2));
}

function nft_output_id(payload: PayloadTypes): string {
    if (payload.type === TRANSACTION_PAYLOAD_TYPE) {
        const txPayload: ITransactionPayload = payload;
        const txHash = Converter.bytesToHex(TransactionHelper.getTransactionPayloadHash(txPayload), true);

        if (txPayload.essence.type === TRANSACTION_ESSENCE_TYPE) {
            const outputs = txPayload.essence.outputs;
            for (let index in txPayload.essence.outputs) {
                if (outputs[index].type === NFT_OUTPUT_TYPE) {
                    const outputId: string = TransactionHelper.outputIdFromTransactionData(txHash, parseInt(index));
                    return TransactionHelper.resolveIdFromOutputId(outputId);
                }
            }
            throw new Error("no NFT output in transaction essence");
        } else {
            throw new Error("expected transaction essence");
        }
    } else {
        throw new Error("expected transaction payload");
    }
}
