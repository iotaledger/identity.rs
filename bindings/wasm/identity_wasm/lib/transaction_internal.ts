// Copyright 2021-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaTransactionBlockResponse } from "@iota/iota-sdk/client";
import { IdentityClient } from "~identity_wasm";

export interface TransactionInternalOutput<T> {
    response: IotaTransactionBlockResponse;
    output: T;
}

export interface TransactionInternal<T> {
    set gasBudget(value: bigint);
    withGasBudget(gasBudget: bigint): TransactionInternal<T>;
    execute(client: IdentityClient): Promise<TransactionInternalOutput<T>>;
}
