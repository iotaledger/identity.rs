// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ApproveUpdateDidDocumentProposal, CreateUpdateDidProposal, ExecuteUpdateDidProposal, IdentityClient, UpdateDid, /*, SendAction, UpdateDid, ConfigChange */ } from "~identity_wasm";
import { TransactionBuilder } from "./transaction_internal";

export type Action = UpdateDid; //| SendAction | ConfigChange;

export type ApproveProposal = ApproveUpdateDidDocumentProposal;
export type ExecuteProposal<A extends Action> = A extends UpdateDid ? ExecuteUpdateDidProposal
    : never;
export type ExecuteProposalOutput<E extends ExecuteProposal<Action>> = E extends ExecuteProposal<UpdateDid> ? ProposalOutput<UpdateDid>
    : never;

export type CreateProposal<A extends Action> = A extends UpdateDid ? CreateUpdateDidProposal
    : never;
export type CreateProposalOutput<C extends CreateProposal<Action>> = C extends CreateProposal<UpdateDid> ? Proposal<UpdateDid> | ProposalOutput<UpdateDid>
    : never;

export type ProposalResult<A extends Action> = ProposalOutput<A> | Proposal<A>;

export type ProposalOutput<A extends Action> = A extends UpdateDid ? void
    : never;
export interface Proposal<A extends Action> {
    id: string;
    get action(): A;
    votes: bigint;
    voters: Set<string>;
    expirationEpoch?: bigint;
    approve: (client: IdentityClient) => TransactionBuilder<ApproveProposal>;
    intoTx: (client: IdentityClient) => TransactionBuilder<ExecuteProposal<A>>;
}
