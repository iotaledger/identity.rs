// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaObjectRef, IotaTransactionBlockResponse, TransactionEffects } from "@iota/iota-sdk/client";
import { TransactionDataBuilder } from "@iota/iota-sdk/transactions";
import { IdentityClient, IdentityClientReadOnly } from "~identity_wasm";

export interface TransactionOutput<T extends Transaction<unknown>> {
    response: IotaTransactionBlockResponse;
    output: Awaited<ReturnType<T["apply"]>>;
}

export interface Transaction<Output> {
    buildProgrammableTransaction(client: IdentityClientReadOnly): Promise<Uint8Array>;
    apply(effects: TransactionEffects, client: IdentityClientReadOnly): Promise<Output>;
}

export type SponsorFn = (tx_data: TransactionDataBuilder) => Promise<string>;

export interface TransactionBuilder<T extends Transaction<unknown>> {
    get transaction(): Readonly<Transaction<T>>;
    withGasPrice(price: bigint): TransactionBuilder<T>;
    withGasBudget(budget: bigint): TransactionBuilder<T>;
    withGasOwner(owner: string): TransactionBuilder<T>;
    withGasPayment(payment: IotaObjectRef[]): TransactionBuilder<T>;
    withSender(sender: String): TransactionBuilder<T>;
    withSignature(client: IdentityClient): TransactionBuilder<T>;
    withSponsor(client: IdentityClientReadOnly, sponsorFn: SponsorFn): Promise<TransactionBuilder<T>>;
    build(client: IdentityClient): Promise<[Uint8Array, string[], T]>;
    buildAndExecute(client: IdentityClient): Promise<TransactionOutput<T>>;
}
