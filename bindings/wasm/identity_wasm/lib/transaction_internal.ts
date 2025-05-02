// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaObjectRef, IotaTransactionBlockResponse, TransactionEffects } from "@iota/iota-sdk/client";
import { TransactionDataBuilder } from "@iota/iota-sdk/transactions";
import { CoreClient, CoreClientReadOnly } from "core_client";
import { TransactionSigner } from "~identity_wasm";

export interface TransactionOutput<T extends Transaction<unknown>> {
    response: IotaTransactionBlockResponse;
    output: Awaited<ReturnType<T["apply"]>>;
}

export interface Transaction<Output> {
    buildProgrammableTransaction(client: CoreClientReadOnly): Promise<Uint8Array>;
    apply(effects: TransactionEffects, client: CoreClientReadOnly): Promise<Output>;
}

export type SponsorFn = (tx_data: TransactionDataBuilder) => Promise<string>;

export interface TransactionBuilder<T extends Transaction<unknown>> {
    get transaction(): Readonly<Transaction<T>>;
    withGasPrice(price: bigint): TransactionBuilder<T>;
    withGasBudget(budget: bigint): TransactionBuilder<T>;
    withGasOwner(owner: string): TransactionBuilder<T>;
    withGasPayment(payment: IotaObjectRef[]): TransactionBuilder<T>;
    withSender(sender: String): TransactionBuilder<T>;
    withSignature<S extends TransactionSigner>(client: CoreClient<S>): TransactionBuilder<T>;
    withSponsor(client: CoreClientReadOnly, sponsorFn: SponsorFn): Promise<TransactionBuilder<T>>;
    build<S extends TransactionSigner>(client: CoreClient<S>): Promise<[Uint8Array, string[], T]>;
    buildAndExecute<S extends TransactionSigner>(client: CoreClient<S>): Promise<TransactionOutput<T>>;
}
