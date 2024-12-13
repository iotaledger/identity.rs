// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { blake2b } from '@noble/hashes/blake2b';
import {
    CoinStruct,
    ExecutionStatus,
    IotaClient,
    IotaTransactionBlockResponse,
    OwnedObjectRef,
} from "@iota/iota.js/client";
import { messageWithIntent, toSerializedSignature } from "@iota/iota.js/cryptography";
import { Ed25519PublicKey } from '@iota/iota.js/keypairs/ed25519';
import { TransactionDataBuilder } from "@iota/iota.js/transactions";

export type Signer = { sign(data: Uint8Array): Promise<Uint8Array> };

export class IotaTransactionBlockResponseAdapter {
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
        return this.response.effects != null && this.response.effects.created != null ? this.response.effects.created : null;
    }
}

async function signTransactionData(
    txBcs: Uint8Array,
    senderPublicKey: Uint8Array,
    signer: { sign(data: Uint8Array): Promise<Uint8Array> },
): Promise<string> {
    const intent = 'TransactionData';
    const intentMessage = messageWithIntent(intent, txBcs);
    const digest = blake2b(intentMessage, { dkLen: 32 });
    const signerSignature = await signer.sign(digest);
    const signature = toSerializedSignature({
        signature: await signerSignature,
        signatureScheme: 'ED25519',
        publicKey: new Ed25519PublicKey(senderPublicKey),
    });

    return signature;
}

async function getCoinForTransaction(iotaClient: IotaClient, senderAddress: string):  Promise<CoinStruct> {
    const coins = await iotaClient.getCoins({ owner: senderAddress });
    if (coins.data.length === 0) {
        throw new Error("could not find coins for transaction");
    }
    
    return coins.data[1];
}

async function addGasDataToTransaction(
    iotaClient: IotaClient,
    senderAddress: string,
    txBcs: Uint8Array,
    gasBudget?: bigint,
): Promise<Uint8Array> {
    const gasPrice = await iotaClient.getReferenceGasPrice();
    const gasCoin = await getCoinForTransaction(iotaClient, senderAddress);
    const txData = TransactionDataBuilder.fromBytes(txBcs);
    const gasData = {
        budget: gasBudget ? gasBudget.toString() : "50000000000", // 50_000_000_000
        owner: senderAddress,
        payment: [{
            objectId: gasCoin.coinObjectId,
            version: gasCoin.version,
            digest: gasCoin.digest,
        }],
        price: gasPrice.toString(),
    };
    let builtTx = txData.build({
        overrides: {
            gasData,
            sender: senderAddress,
        }});

    if (!gasBudget) {
        // no budget given, so we have to estimate gas usage
        const dryRunGasResult = (await iotaClient
            .dryRunTransactionBlock({ transactionBlock: builtTx })).effects;
        if (dryRunGasResult.status.status === 'failure') {
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

        gasData.budget = budget.toString();
        builtTx = txData.build({
            overrides: {
                gasData,
                sender: senderAddress,
            }});
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
): Promise<IotaTransactionBlockResponseAdapter> {
    const txWithGasData = await addGasDataToTransaction(iotaClient, senderAddress, txBcs, gasBudget);
    const signature = await signTransactionData(txWithGasData, senderPublicKey, signer);
    console.log(signature);

    const response = await iotaClient.executeTransactionBlock({
        transactionBlock: txWithGasData,
        signature,
        options: { // `IotaTransactionBlockResponseOptions::full_content()`
            showEffects: true,
            showInput: true,
            showRawInput: true,
            showEvents: true,
            showObjectChanges: true,
            showBalanceChanges: true,
            showRawEffects: false, 
        },
    });
    console.dir(response);

    if (response?.effects?.status.status === 'failure') {
        throw new Error(`transaction returned an unexpected response; ${response?.effects?.status.error}`);
    }

    return new IotaTransactionBlockResponseAdapter(response);
}
