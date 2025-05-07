// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function approve(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    proposalId: string,
    proposalType: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const proposal = tx.pure.id(proposalId);

    tx.moveCall({
        target: `${packageId}::identity::approve_proposal`,
        typeArguments: [proposalType],
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}
