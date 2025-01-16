// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Transaction, TransactionArgument } from "@iota/iota-sdk/transactions"

export function new_(
  inner_bytes: Uint8Array,
  inner_type: string,
  mutable: boolean,
  transferable: boolean,
  deletable: boolean,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const inner_arg = tx.pure(inner_bytes)
  const mutable_arg = tx.pure.bool(mutable);
  const transferable_arg = tx.pure.bool(transferable);
  const deletable_arg = tx.pure.bool(deletable);

  tx.moveCall({
    target: `${packageId}::asset::new_with_config`,
    typeArguments: [inner_type],
    arguments: [inner_arg, mutable_arg, transferable_arg, deletable_arg]
  });

  return tx.build();
}