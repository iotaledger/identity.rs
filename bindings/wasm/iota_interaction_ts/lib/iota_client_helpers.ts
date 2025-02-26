// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinStruct,
    ExecutionStatus,
    IotaClient,
    IotaTransactionBlockResponse,
    OwnedObjectRef,
} from "@iota/iota-sdk/client";
import { messageWithIntent, toSerializedSignature } from "@iota/iota-sdk/cryptography";
import { Ed25519PublicKey } from "@iota/iota-sdk/keypairs/ed25519";
import { GasData, TransactionDataBuilder } from "@iota/iota-sdk/transactions";
import { blake2b } from "@noble/hashes/blake2b";

export type Signer = { sign(data: Uint8Array): Promise<Uint8Array> };

export class IotaTransactionBlockResponseWrapper {
    response: IotaTransactionBlockResponse;

    constructor(response: IotaTransactionBlockResponse) {
        this.response = response;
    }

    effects_is_none(): boolean {
        return this.response.effects == null;
    }

    effects_is_some(): boolean {
        return !(typeof this.response.effects == null);
    }

    to_string(): string {
        return JSON.stringify(this.response);
    }

    effects_execution_status_inner(): null | ExecutionStatus {
        return this.response.effects != null ? this.response.effects.status : null;
    }

    effects_created_inner(): null | OwnedObjectRef[] {
        return this.response.effects != null && this.response.effects.created != null
            ? this.response.effects.created
            : null;
    }

    get_response(): IotaTransactionBlockResponse {
        return this.response;
    }

    get_digest(): string {
        return this.response.digest;
    }
}

/**
 * Builds message with `TransactionData` intent and returns hash of it.
 *
 * @param txBcs transaction data to hash
 * @returns digest/hash of intent message for transaction data
 */
export function getTransactionDigest(txBcs: Uint8Array): Uint8Array {
    const intent = "TransactionData";
    const intentMessage = messageWithIntent(intent, txBcs);
    return blake2b(intentMessage, { dkLen: 32 });
}

async function signTransactionData(
    txBcs: Uint8Array,
    senderPublicKey: Uint8Array,
    signer: { sign(data: Uint8Array): Promise<Uint8Array> },
): Promise<string> {
    const digest = getTransactionDigest(txBcs);
    const signerSignature = await signer.sign(digest);
    const signature = toSerializedSignature({
        signature: await signerSignature,
        signatureScheme: "ED25519",
        publicKey: new Ed25519PublicKey(senderPublicKey),
    });

    return signature;
}

async function getCoinForTransaction(iotaClient: IotaClient, senderAddress: string): Promise<CoinStruct> {
    const coins = await iotaClient.getCoins({ owner: senderAddress });
    if (coins.data.length === 0) {
        throw new Error(`could not find coins for transaction with sender ${senderAddress}`);
    }

    return coins.data[1];
}

/**
 * Inserts these values into the transaction and replaces placeholder values.
 *
 *   - sender (overwritten as we assume a placeholder to be used in prepared transaction)
 *   - gas budget (value determined automatically if not provided)
 *   - gas price (value determined automatically)
 *   - gas coin / payment object (fetched automatically)
 *   - gas owner (equals sender)
 *
 * @param iotaClient client instance
 * @param senderAddress transaction sender (and the one paying for it)
 * @param txBcs transaction data serialized to bcs, most probably having placeholder values
 * @param gasBudget optional fixed gas budget, determined automatically with a dry run if not provided
 * @returns updated transaction data
 */
export async function addGasDataToTransaction(
    iotaClient: IotaClient,
    senderAddress: string,
    txBcs: Uint8Array,
    gasBudget?: bigint,
): Promise<Uint8Array> {
    const gasPrice = await iotaClient.getReferenceGasPrice();
    const gasCoin = await getCoinForTransaction(iotaClient, senderAddress);
    const txData = TransactionDataBuilder.fromBytes(txBcs);
    const gasData: GasData = {
        budget: gasBudget ? gasBudget.toString() : "50000000000", // 50_000_000_000
        owner: senderAddress,
        payment: [{
            objectId: gasCoin.coinObjectId,
            version: gasCoin.version,
            digest: gasCoin.digest,
        }],
        price: gasPrice.toString(),
    };
    const overrides = {
        gasData,
        sender: senderAddress,
    };
    // TODO: check why `.build` with `overrides` doesn't override these values
    txData.sender = overrides.sender;
    txData.gasData = overrides.gasData;
    let builtTx = txData.build({ overrides });

    if (!gasBudget) {
        // no budget given, so we have to estimate gas usage
        const dryRunGasResult = (await iotaClient
            .dryRunTransactionBlock({ transactionBlock: builtTx })).effects;
        if (dryRunGasResult.status.status === "failure") {
            throw new Error("transaction returned an unexpected response; " + dryRunGasResult.status.error);
        }

        const gasSummary = dryRunGasResult.gasUsed;
        const overhead = gasPrice * BigInt(1000);
        let netUsed = BigInt(gasSummary.computationCost)
            + BigInt(gasSummary.storageCost)
            - BigInt(gasSummary.storageRebate);
        netUsed = netUsed >= 0 ? netUsed : BigInt(0);
        const computation = BigInt(gasSummary.computationCost);
        const maxCost = netUsed > computation ? netUsed : computation;
        const budget = overhead + maxCost;

        overrides.gasData.budget = budget.toString();
        txData.gasData.budget = budget.toString();

        builtTx = txData.build({ overrides });
    }

    return builtTx;
}

// estimate gas, get coin, execute tx here
export async function executeTransaction(
    iotaClient: IotaClient,
    senderAddress: string,
    senderPublicKey: Uint8Array,
    txBcs: Uint8Array,
    signer: Signer,
    gasBudget?: bigint,
): Promise<IotaTransactionBlockResponseWrapper> {
    const txWithGasData = await addGasDataToTransaction(iotaClient, senderAddress, txBcs, gasBudget);
    const signature = await signTransactionData(txWithGasData, senderPublicKey, signer);
    console.log(signature);

    const response = await iotaClient.executeTransactionBlock({
        transactionBlock: txWithGasData,
        signature,
        options: { // equivalent of `IotaTransactionBlockResponseOptions::full_content()`
            showEffects: true,
            showInput: true,
            showRawInput: true,
            showEvents: true,
            showObjectChanges: true,
            showBalanceChanges: true,
            showRawEffects: false,
        },
    });

    if (response?.effects?.status.status === "failure") {
        throw new Error(`transaction returned an unexpected response; ${response?.effects?.status.error}`);
    }

    return new IotaTransactionBlockResponseWrapper(response);
}

/**
 * Helper function to pause execution.
 *
 * @param txBcs transaction data to hash
 */
export function sleep(durationMs: number) {
    return new Promise(resolve => setTimeout(resolve, durationMs));
}
