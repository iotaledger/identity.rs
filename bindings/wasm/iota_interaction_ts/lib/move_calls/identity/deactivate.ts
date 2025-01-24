// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota.js/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota.js/transactions";
import { getClockRef, getControllerDelegation, putBackDelegationToken } from "../utils";

export function proposeDeactivation(
    identity: SharedObjectRef,
    capability: ObjectRef,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::propose_deactivation`,
        arguments: [identityArg, delegationToken, exp, clock],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}

export function executeDeactivation(
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
        target: `${packageId}::identity::execute_deactivation`,
        arguments: [identityArg, delegationToken, proposal, clock],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}
