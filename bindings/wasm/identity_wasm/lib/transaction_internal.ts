// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaTransactionBlockResponse, TransactionEffects, Signature, IotaObjectRef } from "@iota/iota-sdk/client";
import { TransactionData, GasData } from "@iota/iota-sdk/transactions";
import { IdentityClientReadOnly, PublishDidDocument, IotaDocument, IdentityClient, CreateIdentity, OnChainIdentity } from "~identity_wasm";
import { Action, ApproveProposal, CreateProposal, CreateProposalOutput, ExecuteProposal, ExecuteProposalOutput } from "./proposal";

type Tx = PublishDidDocument
    | CreateIdentity
    | ApproveProposal
    | CreateProposal<Action>
    | ExecuteProposal<Action>;

type TxOutput<T extends Tx> = T extends PublishDidDocument ? IotaDocument
    : T extends CreateIdentity ? OnChainIdentity
    : T extends ApproveProposal ? void
    : T extends CreateProposal<Action> ? CreateProposalOutput<T>
    : T extends ExecuteProposal<Action> ? ExecuteProposalOutput<T>
    : never;

export interface TransactionOutput<T extends Tx> {
    response: IotaTransactionBlockResponse;
    output: TxOutput<T>;
}

export interface Transaction<T extends Tx> {
    buildProgrammableTransaction(client: IdentityClientReadOnly): Promise<Uint8Array>;
    apply(effects: TransactionEffects, client: IdentityClientReadOnly): Promise<TxOutput<T>>;
}

export type TransactionDataMutGas = Readonly<Omit<TransactionData, "gasData">> & { gasData: GasData };
export type SponsorFn = (txData: TransactionDataMutGas) => Signature;

export interface TransactionBuilder<T extends Tx> {
    get transaction(): Readonly<Transaction<T>>;
    withGasPrice(price: bigint): TransactionBuilder<T>;
    withGasBudget(budget: bigint): TransactionBuilder<T>;
    withGasOwner(owner: string): TransactionBuilder<T>;
    withGasPayment(payment: IotaObjectRef[]): TransactionBuilder<T>;
    withSender(sender: String): TransactionBuilder<T>;
    withSignature(client: IdentityClient): TransactionBuilder<T>;
    withSponsor(client: IdentityClientReadOnly, sponsorFn: SponsorFn): TransactionBuilder<T>;
    build(client: IdentityClient): Promise<[TransactionData, Signature[], Transaction<T>]>;
    buildAndExecute(client: IdentityClient): Promise<TransactionOutput<T>>;
}
