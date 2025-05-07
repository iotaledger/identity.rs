// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function proposeConfigChange(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    controllersToAdd: [string, number][],
    controllersToRemove: string[],
    controllersToUpdate: [string, number][],
    packageId: string,
    expiration?: number,
    threshold?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const addressesToAdd = tx.pure.vector("address", controllersToAdd.map(c => c[0]));
    const vpsToAdd = tx.pure.vector("u64", controllersToAdd.map(c => c[1]));
    const controllersToAddArg = tx.moveCall({
        target: `${packageId}::utils::vec_map_from_keys_values`,
        typeArguments: ["address", "u64"],
        arguments: [addressesToAdd, vpsToAdd],
    });

    const idsToUpdate = tx.pure.vector("id", controllersToUpdate.map(c => c[0]));
    const vpsToUpdate = tx.pure.vector("u64", controllersToUpdate.map(c => c[1]));
    const controllersToUpdateArg = tx.moveCall({
        target: `${packageId}::utils::vec_map_from_keys_values`,
        typeArguments: ["id", "u64"],
        arguments: [idsToUpdate, vpsToUpdate],
    });

    const identityArg = tx.sharedObjectRef(identity);
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const thresholdArg = tx.pure.option("u64", threshold);
    const exp = tx.pure.option("u64", expiration);
    const controllersToRemoveArg = tx.pure.vector("id", controllersToRemove);

    tx.moveCall({
        target: `${packageId}::identity::propose_config_change`,
        arguments: [identityArg, cap.token, exp, thresholdArg, controllersToAddArg, controllersToRemoveArg,
            controllersToUpdateArg],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}

export function executeConfigChange(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    proposalId: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const proposal = tx.pure.id(proposalId);
    const identityArg = tx.sharedObjectRef(identity);

    tx.moveCall({
        target: `${packageId}::identity::execute_config_change`,
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}
