// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { bcs } from "@iota/iota-sdk/bcs";
import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";

export interface ControllerTokenRef {
    objectRef: ObjectRef;
    type: "ControllerCap" | "DelegationToken";
}

export async function delegateControllerCap(
    controllerCap: ObjectRef,
    recipient: string,
    permissions: number,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const cap = tx.objectRef(controllerCap);
    const recipientAddress = tx.pure.address(recipient);
    const permissionsArg = tx.pure.u32(permissions);

    const delegationToken = tx.moveCall({
        target: `${packageId}::controller::delegate_with_permissions`,
        arguments: [cap, permissionsArg],
    });

    tx.transferObjects([delegationToken], recipientAddress);

    return tx.build({ onlyTransactionKind: true });
}

export async function revokeDelegationToken(
    identity: SharedObjectRef,
    controllerCap: ObjectRef,
    delegationTokenId: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();

    const identityArg = tx.sharedObjectRef(identity);
    const controllerCapArg = tx.objectRef(controllerCap);
    const tokenIdArg = tx.pure.id(delegationTokenId);

    tx.moveCall({
        target: `${packageId}::identity::revoke_delegation_token`,
        arguments: [identityArg, controllerCapArg, tokenIdArg],
    });

    return tx.build({ onlyTransactionKind: true });
}

export async function unrevokeDelegationToken(
    identity: SharedObjectRef,
    controllerCap: ObjectRef,
    delegationTokenId: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();

    const identityArg = tx.sharedObjectRef(identity);
    const controllerCapArg = tx.objectRef(controllerCap);
    const tokenIdArg = tx.pure.id(delegationTokenId);

    tx.moveCall({
        target: `${packageId}::identity::unrevoke_delegation_token`,
        arguments: [identityArg, controllerCapArg, tokenIdArg],
    });

    return tx.build({ onlyTransactionKind: true });
}

export async function destroyDelegationToken(
    identity: SharedObjectRef,
    delegationToken: ObjectRef,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();

    const identityArg = tx.sharedObjectRef(identity);
    const tokenArg = tx.objectRef(delegationToken);

    tx.moveCall({
        target: `${packageId}::identity::destroy_delegation_token`,
        arguments: [identityArg, tokenArg],
    });

    return tx.build({ onlyTransactionKind: true });
}
