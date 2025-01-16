// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";

export function proposeUpgrade(
  identity: SharedObjectRef,
  capability: ObjectRef,
  packageId: string,
  expiration?: number,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const identityArg = tx.sharedObjectRef(identity);
  const exp = tx.pure.option('u64', expiration);

  tx.moveCall({
    target: `${packageId}::identity::propose_upgrade`,
    arguments: [identityArg, cap, exp],
  });

  return tx.build();
}

export function executeUpgrade(
  identity: SharedObjectRef,
  capability: ObjectRef,
  proposalId: string,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const proposal = tx.pure.id(proposalId);
  const identityArg = tx.sharedObjectRef(identity);

  tx.moveCall({
    target: `${packageId}::identity::execute_upgrade`,
    arguments: [identityArg, cap, proposal],
  });

  return tx.build();
}

