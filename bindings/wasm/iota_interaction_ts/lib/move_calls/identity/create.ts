// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { bcs } from "@iota/iota-sdk/bcs";
import { Transaction } from "@iota/iota-sdk/transactions";
import { getClockRef } from "../utils";

export async function create(didDoc: Uint8Array | undefined, packageId: string): Promise<Uint8Array> {
    const tx = new Transaction();
    const didDocArg = tx.pure(bcs.option(bcs.vector(bcs.U8)).serialize(didDoc));
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::new`,
        arguments: [didDocArg, clock],
    });

    return tx.build({ onlyTransactionKind: true });
}

export async function newWithControllers(
    didDoc: Uint8Array | undefined,
    controllers: [string, number, boolean][],
    threshold: number,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const controllers_no_delegate = controllers.filter(([_addr, _vp, can_delegate]) => !can_delegate);
    const controllers_delegate = controllers.filter(([_addr, _vp, can_delegate]) => can_delegate);
    const ids = tx.pure.vector("address", controllers_no_delegate.map(controller => controller[0]));
    const vps = tx.pure.vector("u64", controllers_no_delegate.map(controller => controller[1]));

    const ids_delegate = tx.pure.vector("address", controllers_delegate.map(controller => controller[0]));
    const vps_delegate = tx.pure.vector("u64", controllers_delegate.map(controller => controller[1]));
    const controllersArg = tx.moveCall({
        target: `${packageId}::utils::vec_map_from_keys_values`,
        typeArguments: ["address", "u64"],
        arguments: [ids, vps],
    });
    const controllersThatCanDelegate = tx.moveCall({
        target: `${packageId}::utils::vec_map_from_keys_values`,
        typeArguments: ["address", "u64"],
        arguments: [ids_delegate, vps_delegate],
    });
    const didDocArg = tx.pure(bcs.option(bcs.vector(bcs.U8)).serialize(didDoc));
    const clock = getClockRef(tx);
    const thresholdArg = tx.pure.u64(threshold);

    tx.moveCall({
        target: `${packageId}::identity::new_with_controllers`,
        arguments: [didDocArg, controllersArg, controllersThatCanDelegate, thresholdArg, clock],
    });

    return await tx.build({ onlyTransactionKind: true });
}
