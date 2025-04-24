// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { IotaObjectData } from "@iota/iota-sdk/dist/cjs/client";
import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";
import { getControllerDelegation, putBackDelegationToken } from "../utils";

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
    const exp = tx.pure.option("u64", expiration);
    const objectsArg = tx.pure.vector("id", objects);

    tx.moveCall({
        target: `${packageId}::identity::propose_borrow`,
        arguments: [identityArg, delegationToken, exp, objectsArg],
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

    const objectArgMap = new Map<string, [TransactionArgument, IotaObjectData]>();
    for (const obj of objects) {
        const recvObj = tx.receivingRef(obj);
        const objArg = tx.moveCall({
            target: `${packageId}::identity::execute_borrow`,
            typeArguments: [obj.type!],
            arguments: [identityArg, action, recvObj],
        });

        objectArgMap.set(obj.objectId, [objArg, obj]);
    }

    intentFn(tx, objectArgMap);

    for (const [obj, objData] of objectArgMap.values()) {
        tx.moveCall({
            target: `${packageId}::borrow_proposal::put_back`,
            typeArguments: [objData.type!],
            arguments: [action, obj],
        });
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
    const exp = tx.pure.option("u64", expiration);
    const objectsArg = tx.pure.vector("id", objects.map(obj => obj.objectId));

    const proposal = tx.moveCall({
        target: `${packageId}::identity::propose_borrow`,
        arguments: [identityArg, delegationToken, exp, objectsArg],
    });

    let action = tx.moveCall({
        target: `${packageId}::identity::execute_proposal`,
        typeArguments: [`${packageId}::borrow_proposal::Borrow`],
        arguments: [identityArg, delegationToken, proposal],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    const objectArgMap = new Map<string, [TransactionArgument, IotaObjectData]>();
    for (const obj of objects) {
        const recvObj = tx.receivingRef(obj);
        const objArg = tx.moveCall({
            target: `${packageId}::identity::execute_borrow`,
            typeArguments: [obj.type!],
            arguments: [identityArg, action, recvObj],
        });

        objectArgMap.set(obj.objectId, [objArg, obj]);
    }

    intentFn(tx, objectArgMap);

    for (const [obj, objData] of objectArgMap.values()) {
        tx.moveCall({
            target: `${packageId}::borrow_proposal::put_back`,
            typeArguments: [objData.type!],
            arguments: [action, obj],
        });
    }

    tx.moveCall({
        target: `${packageId}::transfer_proposal::conclude_borrow`,
        arguments: [action],
    });
    return tx.build();
}
