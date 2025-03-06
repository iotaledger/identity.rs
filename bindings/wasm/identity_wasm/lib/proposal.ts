// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ConfigChange, IdentityClient, SendAction, UpdateDid } from "~identity_wasm";
import { TransactionInternal } from "./transaction_internal";

type Action = UpdateDid | SendAction | ConfigChange;

export type ProposalOutput<A extends Action> = A extends UpdateDid ? void
    : A extends SendAction ? void
    : A extends ConfigChange ? void
    : never;

export interface Proposal<A extends Action> {
    id: string;
    get action(): A;
    votes: bigint;
    voters: Set<string>;
    expirationEpoch?: bigint;
    approve: (client: IdentityClient) => TransactionInternal<void>;
    intoTx: (client: IdentityClient) => TransactionInternal<ProposalOutput<A>>;
}
