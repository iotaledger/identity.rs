// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaTransactionBlockResponse, TransactionEffects, Signature, IotaObjectRef } from "@iota/iota-sdk/client";
import { TransactionData, GasData } from "@iota/iota-sdk/transactions";
import { IdentityClientReadOnly, IdentityClient } from "~identity_wasm";

export interface TransactionOutput<T extends Transaction<unknown>> {
    response: IotaTransactionBlockResponse;
    output: Awaited<ReturnType<T['apply']>>;
}

export interface Transaction<Output> {
    buildProgrammableTransaction(client: IdentityClientReadOnly): Promise<Uint8Array>;
    apply(effects: TransactionEffects, client: IdentityClientReadOnly): Promise<Output>;
}

export type TransactionDataMutGas = Readonly<Omit<TransactionData, "gasData">> & { gasData: GasData };
export type SponsorFn = (txData: TransactionDataMutGas) => Signature;

export interface TransactionBuilder<T extends Transaction<unknown>> {
    get transaction(): Readonly<Transaction<T>>;
    withGasPrice(price: bigint): TransactionBuilder<T>;
    withGasBudget(budget: bigint): TransactionBuilder<T>;
    withGasOwner(owner: string): TransactionBuilder<T>;
    withGasPayment(payment: IotaObjectRef[]): TransactionBuilder<T>;
    withSender(sender: String): TransactionBuilder<T>;
    withSignature(client: IdentityClient): TransactionBuilder<T>;
    withSponsor(client: IdentityClientReadOnly, sponsorFn: SponsorFn): TransactionBuilder<T>;
    build(client: IdentityClient): Promise<[TransactionData, Signature[], T]>;
    buildAndExecute(client: IdentityClient): Promise<TransactionOutput<T>>;
}
