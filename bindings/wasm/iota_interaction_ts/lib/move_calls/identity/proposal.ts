// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota.js/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota.js/transactions";
import { getControllerDelegation, putBackDelegationToken } from "../utils";

export function approve(
    identity: SharedObjectRef,
    capability: ObjectRef,
    proposalId: string,
    proposalType: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(capability);
    const [delegationToken, borrow] = getControllerDelegation(tx, cap, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const proposal = tx.pure.id(proposalId);

    tx.moveCall({
        target: `${packageId}::identity::approve_proposal`,
        typeArguments: [proposalType],
        arguments: [identityArg, delegationToken, proposal],
    });

    putBackDelegationToken(tx, cap, delegationToken, borrow, packageId);

    return tx.build();
}
