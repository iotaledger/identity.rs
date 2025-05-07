// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function proposeSend(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    transferMap: [string, string][],
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const objects = tx.pure.vector("id", transferMap.map(t => t[0]));
    const recipients = tx.pure.vector("address", transferMap.map(t => t[1]));

    tx.moveCall({
        target: `${packageId}::identity::propose_send`,
        arguments: [identityArg, cap.token, exp, objects, recipients],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}

export function executeSend(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    proposalId: string,
    objects: [ObjectRef, string][],
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const proposal = tx.pure.id(proposalId);
    const identityArg = tx.sharedObjectRef(identity);

    let action = tx.moveCall({
        target: `${packageId}::identity::execute_proposal`,
        typeArguments: [`${packageId}::transfer_proposal::Send`],
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    for (const [obj, objType] of objects) {
        const recv_obj = tx.receivingRef(obj);
        tx.moveCall({
            target: `${packageId}::identity::execute_send`,
            typeArguments: [objType],
            arguments: [identityArg, action, recv_obj],
        });
    }

    tx.moveCall({
        target: `${packageId}::transfer_proposal::complete_send`,
        arguments: [action],
    });

    return tx.build();
}
