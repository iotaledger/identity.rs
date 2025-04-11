// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoinStruct,
    IotaClient,
    IotaTransactionBlockResponse,
    TransactionEffects,
} from "@iota/iota-sdk/client";
import { GasData, TransactionDataBuilder } from "@iota/iota-sdk/transactions";

export type Signer = { sign(data: Uint8Array): Promise<string> };

const MINIMUM_BALANCE_FOR_COIN = BigInt(1_000_000_000);

export class WasmIotaTransactionBlockResponseWrapper {
    response: IotaTransactionBlockResponse;

    constructor(response: IotaTransactionBlockResponse) {
        this.response = response;
    }

    to_string(): string {
        return JSON.stringify(this.response);
    }

    get_effects(): TransactionEffects | null | undefined {
        return this.response.effects;
    }

    get_response(): IotaTransactionBlockResponse {
        return this.response;
    }

    get_digest(): string {
        return this.response.digest;
    }
}

function byHighestBalance<T extends { balance: BigInt }>({ balance: a }: T, { balance: b }: T) {
    if (a > b) {
        return -1;
    }
    if (a < b) {
        return 1;
    }
    return 0;
}

async function getCoinForTransaction(iotaClient: IotaClient, senderAddress: string): Promise<CoinStruct> {
    let cursor: string | null | undefined = undefined;
    do {
        const response = await iotaClient.getCoins({ owner: senderAddress, cursor });
        if (response.data.length === 0) {
            throw new Error(
                `no coin found with minimum required balance of ${MINIMUM_BALANCE_FOR_COIN} for address ${senderAddress}"`,
            );
        }

        let sortedValidCoins = response.data
            .map((coin) => ({ coin, balance: BigInt(coin.balance) }))
            .filter(({ balance }) => balance >= MINIMUM_BALANCE_FOR_COIN)
            .sort(byHighestBalance);

        if (sortedValidCoins.length >= 1) {
            return sortedValidCoins[0].coin;
        }

        cursor = response.nextCursor;
    } while (cursor);

    throw new Error(
        `no coin found with minimum required balance of ${MINIMUM_BALANCE_FOR_COIN} for address ${senderAddress}"`,
    );
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
        budget: gasBudget ? gasBudget.toString() : "50000000", // 50_000_000
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
    txBcs: Uint8Array,
    signer: Signer,
    gasBudget?: bigint,
): Promise<WasmIotaTransactionBlockResponseWrapper> {
    const txWithGasData = await addGasDataToTransaction(iotaClient, senderAddress, txBcs, gasBudget);
    const signature = await signer.sign(txWithGasData);

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

    return new WasmIotaTransactionBlockResponseWrapper(response);
}

/**
 * Helper function to pause execution.
 *
 * @param durationMs time to sleep in ms
 */
export function sleep(durationMs: number) {
    return new Promise(resolve => setTimeout(resolve, durationMs));
}
