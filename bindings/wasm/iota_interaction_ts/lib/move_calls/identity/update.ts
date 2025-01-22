// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { getClockRef, getControllerDelegation, putBackDelegationToken } from "../utils";

export function proposeUpdate(
    identity: SharedObjectRef,
    capability: ObjectRef,
    didDoc: Uint8Array,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const doc = tx.pure.vector("u8", didDoc);
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::propose_update`,
        arguments: [identityArg, delegationToken, doc, exp, clock],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}

export function executeUpdate(
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
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::execute_update`,
        arguments: [identityArg, delegationToken, proposal, clock],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}
