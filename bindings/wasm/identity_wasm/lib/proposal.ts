// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IdentityClient } from "~identity_wasm";
import { TransactionInternal } from "./transaction_internal";

export interface Proposal<Action, Output = void> {
    id: string;
    get action(): Action;
    votes: bigint;
    voters: Set<string>;
    expirationEpoch?: bigint;
    approve: (client: IdentityClient) => TransactionInternal<void>;
    intoTx: (client: IdentityClient) => TransactionInternal<Output>;
}
