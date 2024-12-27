// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { getControllerDelegation, putBackDelegationToken } from "../utils";
import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";

export function proposeControllerExecution(
  identity: SharedObjectRef,
  capability: ObjectRef,
  controllerCapId: string,
  packageId: string,
  expiration?: number,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const identityArg = tx.sharedObjectRef(identity);
  const exp = tx.pure.option('u64', expiration);
  const controller_cap_id = tx.pure.id(controllerCapId);

  tx.moveCall({
    target: `${packageId}::identity::propose_controller_execution`,
    arguments: [identityArg, delegationToken, controller_cap_id, exp],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  return tx.build();
}

export function executeControllerExecution(
  identity: SharedObjectRef,
  capability: ObjectRef,
  proposalId: string,
  controllerCapRef: ObjectRef,
  intentFn: (arg0: Transaction, arg1: TransactionArgument) => void,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const proposal = tx.pure.id(proposalId);
  const identityArg = tx.sharedObjectRef(identity);

  let action = tx.moveCall({
    target: `${packageId}::identity::execute_proposal`,
    typeArguments: [`${packageId}::borrow_proposal::Borrow`],
    arguments: [identityArg, delegationToken, proposal],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  const receiving = tx.receivingRef(controllerCapRef);
  const borrowed_controller_cap = tx.moveCall({
    target: `${packageId}::identity::borrow_controller_cap`,
    arguments: [identityArg, action, receiving],
  });

  intentFn(tx, borrowed_controller_cap);

  tx.moveCall({
    target: `${packageId}::controller_proposal::put_back`,
    arguments: [action, borrowed_controller_cap],
  });

  return tx.build();
}


