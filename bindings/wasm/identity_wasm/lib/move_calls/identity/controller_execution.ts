// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";
import { controllerTokenRefToTxArgument, putBackControllerToken } from "../utils";
import { ControllerTokenRef } from "./controller";

export function proposeControllerExecution(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    controllerCapId: string,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const controllerCapIdArg = tx.pure.id(controllerCapId);

    tx.moveCall({
        target: `${packageId}::identity::propose_controller_execution`,
        arguments: [identityArg, cap.token, controllerCapIdArg, exp],
    });

    putBackControllerToken(tx, cap, packageId);

    return tx.build();
}

export function executeControllerExecution(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    proposalId: string,
    controllerCapRef: ObjectRef,
    intentFn: (arg0: Transaction, arg1: TransactionArgument) => void,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const proposal = tx.pure.id(proposalId);
    const identityArg = tx.sharedObjectRef(identity);

    let action = tx.moveCall({
        target: `${packageId}::identity::execute_proposal`,
        typeArguments: [`${packageId}::borrow_proposal::Borrow`],
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    const receiving = tx.receivingRef(controllerCapRef);
    const BorrowedControllerCap = tx.moveCall({
        target: `${packageId}::identity::borrow_controller_cap`,
        arguments: [identityArg, action, receiving],
    });

    intentFn(tx, BorrowedControllerCap);

    tx.moveCall({
        target: `${packageId}::controller_proposal::put_back`,
        arguments: [action, BorrowedControllerCap],
    });

    return tx.build();
}

export function createAndExecuteControllerExecution(
    identity: SharedObjectRef,
    capability: ControllerTokenRef,
    controllerCapRef: ObjectRef,
    intentFn: (arg0: Transaction, arg1: TransactionArgument) => void,
    packageId: string,
    expiration?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = controllerTokenRefToTxArgument(tx, capability, packageId);
    const identityArg = tx.sharedObjectRef(identity);
    const exp = tx.pure.option("u64", expiration);
    const controller_cap_id = tx.pure.id(controllerCapRef.objectId);

    const proposal = tx.moveCall({
        target: `${packageId}::identity::propose_controller_execution`,
        arguments: [identityArg, cap.token, controller_cap_id, exp],
    });

    let action = tx.moveCall({
        target: `${packageId}::identity::execute_proposal`,
        typeArguments: [`${packageId}::borrow_proposal::Borrow`],
        arguments: [identityArg, cap.token, proposal],
    });

    putBackControllerToken(tx, cap, packageId);

    const receiving = tx.receivingRef(controllerCapRef);
    const borrowedControllerCap = tx.moveCall({
        target: `${packageId}::identity::borrow_controller_cap`,
        arguments: [identityArg, action, receiving],
    });

    intentFn(tx, borrowedControllerCap);

    tx.moveCall({
        target: `${packageId}::controller_proposal::put_back`,
        arguments: [action, borrowedControllerCap],
    });

    return tx.build();
}
