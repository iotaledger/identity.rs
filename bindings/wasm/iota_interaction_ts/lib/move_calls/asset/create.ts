// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from "@iota/iota.js/transactions";

export function create(
    inner_bytes: Uint8Array,
    inner_type: string,
    mutable: boolean,
    transferable: boolean,
    deletable: boolean,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const inner_arg = tx.pure(inner_bytes);
    const mutableArg = tx.pure.bool(mutable);
    const transferableArg = tx.pure.bool(transferable);
    const deletableArg = tx.pure.bool(deletable);

    tx.moveCall({
        target: `${packageId}::asset::new_with_config`,
        typeArguments: [inner_type],
        arguments: [inner_arg, mutableArg, transferableArg, deletableArg],
    });

    return tx.build();
}
