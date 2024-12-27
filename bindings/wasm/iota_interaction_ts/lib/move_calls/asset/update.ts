// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions"

export function update(
  asset: ObjectRef,
  content: (tx: Transaction) => TransactionArgument,
  content_type: string,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const content_arg = content(tx);
  const asset_arg = tx.objectRef(asset);

  tx.moveCall({
    target: `${packageId}::asset::update`,
    typeArguments: [content_type],
    arguments: [asset_arg, content_arg]
  });

  return tx.build();
}
