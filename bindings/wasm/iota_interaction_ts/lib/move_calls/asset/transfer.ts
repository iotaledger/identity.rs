// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota-sdk/transactions";

export function transfer(
    asset: ObjectRef,
    assetType: string,
    recipient: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const assetArg = tx.objectRef(asset);
    const recipientArg = tx.pure.address(recipient);

    tx.moveCall({
        target: `${packageId}::asset::transfer`,
        typeArguments: [assetType],
        arguments: [assetArg, recipientArg],
    });

    return tx.build();
}

function makeTx(
    proposal: SharedObjectRef,
    cap: ObjectRef,
    asset: ObjectRef,
    assetType: string,
    packageId: string,
    functionName: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const proposalArg = tx.sharedObjectRef(proposal);
    const capArg = tx.objectRef(cap);
    const assetArg = tx.objectRef(asset);

    tx.moveCall({
        target: `${packageId}::asset::${functionName}`,
        typeArguments: [assetType],
        arguments: [proposalArg, capArg, assetArg],
    });

    return tx.build();
}

export function acceptProposal(
    proposal: SharedObjectRef,
    recipientCap: ObjectRef,
    asset: ObjectRef,
    assetType: string,
    packageId: string,
): Promise<Uint8Array> {
    return makeTx(
        proposal,
        recipientCap,
        asset,
        assetType,
        packageId,
        "accept",
    );
}

export function concludeOrCancel(
    proposal: SharedObjectRef,
    senderCap: ObjectRef,
    asset: ObjectRef,
    assetType: string,
    packageId: string,
): Promise<Uint8Array> {
    return makeTx(
        proposal,
        senderCap,
        asset,
        assetType,
        packageId,
        "conclude_or_cancel",
    );
}
