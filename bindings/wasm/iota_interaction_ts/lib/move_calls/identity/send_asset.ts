// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota.js/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota.js/transactions";
import { getControllerDelegation, putBackDelegationToken } from "../utils";

export function proposeSend(
    identity: SharedObjectRef,
    capability: ObjectRef,
    transferMap: [string, string][],
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const objects = tx.pure.vector("id", transferMap.map(t => t[0]));
    const recipients = tx.pure.vector("address", transferMap.map(t => t[1]));

    tx.moveCall({
        target: `${packageId}::identity::propose_send`,
        arguments: [identityArg, delegationToken, exp, objects, recipients],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}

export function executeSend(
    identity: SharedObjectRef,
    capability: ObjectRef,
    proposalId: string,
    objects: [ObjectRef, string][],
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const proposal = tx.pure.id(proposalId);
    const identityArg = tx.sharedObjectRef(identity);

    let action = tx.moveCall({
        target: `${packageId}::identity::execute_proposal`,
        typeArguments: [`${packageId}::transfer_proposal::Send`],
        arguments: [identityArg, delegationToken, proposal],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

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
