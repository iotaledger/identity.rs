// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { getClockRef, getControllerDelegation, putBackDelegationToken } from "../utils";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";

export function proposeConfigChange(
  identity: SharedObjectRef,
  controllerCap: ObjectRef,
  controllersToAdd: [string, number][],
  controllersToRemove: string[],
  controllersToUpdate: [string, number][],
  packageId: string,
  expiration?: number,
  threshold?: number,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const addresses_to_add = tx.pure.vector('address', controllersToAdd.map(c => c[0]));
  const vps_to_add = tx.pure.vector('u64', controllersToAdd.map(c => c[1]));
  const controllers_to_add = tx.moveCall({
    target: `${packageId}::utils::vec_map_from_keys_values`,
    typeArguments: ['address', 'u64'],
    arguments: [addresses_to_add, vps_to_add],
  });

  const ids_to_update = tx.pure.vector('id', controllersToUpdate.map(c => c[0]));
  const vps_to_update = tx.pure.vector('u64', controllersToUpdate.map(c => c[1]));
  const controllers_to_update = tx.moveCall({
    target: `${packageId}::utils::vec_map_from_keys_values`,
    typeArguments: ['id', 'u64'],
    arguments: [ids_to_update, vps_to_update],
  });

  const identityArg = tx.sharedObjectRef(identity);
  const cap = tx.objectRef(controllerCap);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const thresholdArg = tx.pure.option('u64', threshold);
  const exp = tx.pure.option('u64', expiration);
  const controllers_to_remove = tx.pure.vector('id', controllersToRemove);

  tx.moveCall({
    target: `${packageId}::identity::propose_config_change`,
    arguments: [identityArg, delegationToken, exp, thresholdArg, controllers_to_add, controllers_to_remove, controllers_to_update],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  return tx.build();
}

export function executeConfigChange(
  identity: SharedObjectRef,
  capability: ObjectRef,
  proposalId: string,
  packageId: string,
): Promise<Uint8Array> {
  const tx = new Transaction();
  const cap = tx.objectRef(capability);
  const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
  const proposal = tx.pure.id(proposalId);
  const identityArg = tx.sharedObjectRef(identity);

  tx.moveCall({
    target: `${packageId}::identity::execute_config_change`,
    arguments: [identityArg, delegationToken, proposal],
  });

  putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

  return tx.build();
}