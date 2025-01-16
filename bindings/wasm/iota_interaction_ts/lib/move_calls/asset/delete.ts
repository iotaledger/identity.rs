// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions"

export function delete_(
  asset: ObjectRef,
  asset_type: string,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const asset_arg = tx.objectRef(asset);

  tx.moveCall({
    target: `${packageId}::asset::delete`,
    typeArguments: [asset_type],
    arguments: [asset_arg]
  });

  return tx.build();
}

