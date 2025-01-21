// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { getControllerDelegation, putBackDelegationToken } from "../utils";
import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";
import { IotaObjectData } from "@iota/iota-sdk/dist/cjs/client";

export function proposeBorrow(
  identity: SharedObjectRef,
  capability: ObjectRef,
  objects: string[],
  packageId: string,
  expiration?: number,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const identityArg = tx.sharedObjectRef(identity);
  const exp = tx.pure.option('u64', expiration);
  const objects_arg = tx.pure.vector('id', objects);

  tx.moveCall({
    target: `${packageId}::identity::propose_borrow`,
    arguments: [identityArg, delegationToken, exp, objects_arg],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  return tx.build();
}

export function executeBorrow(
  identity: SharedObjectRef,
  capability: ObjectRef,
  proposalId: string,
  objects: IotaObjectData[],
  intentFn: (arg0: Transaction, arg1: Map<string, [TransactionArgument, IotaObjectData]>) => void,
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

  const object_arg_map = new Map<string, [TransactionArgument, IotaObjectData]>();
  for (const obj of objects) {
    const recv_obj = tx.receivingRef(obj);
    const obj_arg = tx.moveCall({
      target: `${packageId}::identity::execute_borrow`,
      typeArguments: [obj.type!],
      arguments: [identityArg, action, recv_obj],
    });

    object_arg_map.set(obj.objectId, [obj_arg, obj]);
  }

  intentFn(tx, object_arg_map);

  for (const [obj, obj_data] of object_arg_map.values()) {
    tx.moveCall({
      target: `${packageId}::borrow_proposal::put_back`,
      typeArguments: [obj_data.type!],
      arguments: [action, obj],
    })
  }

  tx.moveCall({
    target: `${packageId}::transfer_proposal::conclude_borrow`,
    arguments: [action],
  });

  return tx.build();
}

export function createAndExecuteBorrow(
  identity: SharedObjectRef,
  capability: ObjectRef,
  objects: IotaObjectData[],
  intentFn: (arg0: Transaction, arg1: Map<string, [TransactionArgument, IotaObjectData]>) => void,
  packageId: string,
  expiration?: number,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const identityArg = tx.sharedObjectRef(identity);
  const exp = tx.pure.option('u64', expiration);
  const objects_arg = tx.pure.vector('id', objects.map(obj => obj.objectId));

  const proposal = tx.moveCall({
    target: `${packageId}::identity::propose_borrow`,
    arguments: [identityArg, delegationToken, exp, objects_arg],
  });

  let action = tx.moveCall({
    target: `${packageId}::identity::execute_proposal`,
    typeArguments: [`${packageId}::borrow_proposal::Borrow`],
    arguments: [identityArg, delegationToken, proposal],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  const object_arg_map = new Map<string, [TransactionArgument, IotaObjectData]>();
  for (const obj of objects) {
    const recv_obj = tx.receivingRef(obj);
    const obj_arg = tx.moveCall({
      target: `${packageId}::identity::execute_borrow`,
      typeArguments: [obj.type!],
      arguments: [identityArg, action, recv_obj],
    });

    object_arg_map.set(obj.objectId, [obj_arg, obj]);
  }

  intentFn(tx, object_arg_map);

  for (const [obj, obj_data] of object_arg_map.values()) {
    tx.moveCall({
      target: `${packageId}::borrow_proposal::put_back`,
      typeArguments: [obj_data.type!],
      arguments: [action, obj],
    })
  }

  tx.moveCall({
    target: `${packageId}::transfer_proposal::conclude_borrow`,
    arguments: [action],
  });
  return tx.build();
}
