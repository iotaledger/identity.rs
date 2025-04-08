// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ConfigChange, IdentityClient, SendAction, UpdateDid } from "~identity_wasm";
import { Transaction, TransactionBuilder } from "./transaction_internal";

export type Action = UpdateDid | SendAction | ConfigChange;
export type ProposalOutput<A extends Action> = A extends UpdateDid ? void
    : A extends SendAction ? void
    : A extends ConfigChange ? void
    : never;
export type ProposalResult<A extends Action> = ProposalOutput<A> | Proposal<A>;

export type ApproveProposal = Transaction<void>;
export type ExecuteProposal<A extends Action> = Transaction<ProposalOutput<A>>;
export type CreateProposal<A extends Action> = Transaction<ProposalResult<A>>;
export interface Proposal<A extends Action> {
    id: string;
    get action(): A;
    votes: bigint;
    voters: Set<string>;
    expirationEpoch?: bigint;
    approve: (client: IdentityClient) => TransactionBuilder<ApproveProposal>;
    intoTx: (client: IdentityClient) => TransactionBuilder<ExecuteProposal<A>>;
}
