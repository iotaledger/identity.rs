// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";

export function update(
    asset: ObjectRef,
    content: Uint8Array,
    contentType: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const contentArg = tx.pure(content);
    const assetArg = tx.objectRef(asset);

    tx.moveCall({
        target: `${packageId}::asset::update`,
        typeArguments: [contentType],
        arguments: [assetArg, contentArg],
    });

    return tx.build();
}
