// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota-sdk/dist/cjs/bcs/types";
import { ObjectRef, Transaction, TransactionArgument } from "@iota/iota-sdk/transactions";

export function transfer(
    asset: ObjectRef,
    asset_type: string,
    recipient: string,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const asset_arg = tx.objectRef(asset);
    const recipient_arg = tx.pure.address(recipient);

    tx.moveCall({
        target: `${packageId}::asset::transfer`,
        typeArguments: [asset_type],
        arguments: [asset_arg, recipient_arg],
    });

    return tx.build();
}

function make_tx(
    proposal: SharedObjectRef,
    cap: ObjectRef,
    asset: ObjectRef,
    asset_type: string,
    packageId: string,
    function_name: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const proposal_arg = tx.sharedObjectRef(proposal);
    const cap_arg = tx.objectRef(cap);
    const asset_arg = tx.objectRef(asset);

    tx.moveCall({
        target: `${packageId}::asset::${function_name}`,
        typeArguments: [asset_type],
        arguments: [proposal_arg, cap_arg, asset_arg],
    });

    return tx.build();
}

export function acceptProposal(
    proposal: SharedObjectRef,
    recipient_cap: ObjectRef,
    asset: ObjectRef,
    asset_type: string,
    packageId: string,
): Promise<Uint8Array> {
    return make_tx(
        proposal,
        recipient_cap,
        asset,
        asset_type,
        packageId,
        "accept",
    );
}

export function concludeOrCancel(
    proposal: SharedObjectRef,
    sender_cap: ObjectRef,
    asset: ObjectRef,
    asset_type: string,
    packageId: string,
): Promise<Uint8Array> {
    return make_tx(
        proposal,
        sender_cap,
        asset,
        asset_type,
        packageId,
        "conclude_or_cancel",
    );
}
