// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { bcs } from "@iota/iota-sdk/bcs";
import { getClockRef, getControllerDelegation, insertPlaceholders, putBackDelegationToken } from "../utils";

export function proposeUpdate(
    identity: SharedObjectRef,
    capability: ObjectRef,
    didDoc: Uint8Array | undefined,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const doc = tx.pure(bcs.option(bcs.vector(bcs.U8)).serialize(didDoc));
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::propose_update`,
        arguments: [identityArg, delegationToken, doc, exp, clock],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    insertPlaceholders(tx);

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
