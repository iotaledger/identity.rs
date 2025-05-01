// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { Transaction } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function proposeUpgrade(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);

    tx.moveCall({
        target: `${packageId}::identity::propose_upgrade`,
        arguments: [identityArg, cap.token, exp],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}

export function executeUpgrade(
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
        target: `${packageId}::identity::execute_upgrade`,
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}
