// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { bcs } from "@iota/iota-sdk/bcs";
import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, getClockRef, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function proposeUpdate(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    didDoc: Uint8Array | undefined,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const doc = tx.pure(bcs.option(bcs.vector(bcs.U8)).serialize(didDoc));
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::propose_update`,
        arguments: [identityArg, cap.token, doc, exp, clock],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build({ onlyTransactionKind: true });
}

export function executeUpdate(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    proposalId: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const proposal = tx.pure.id(proposalId);
    const identityArg = tx.sharedObjectRef(identity);
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::execute_update`,
        arguments: [identityArg, cap.token, proposal, clock],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build({ onlyTransactionKind: true });
}
