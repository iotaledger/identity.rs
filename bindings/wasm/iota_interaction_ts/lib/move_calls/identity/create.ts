// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from "@iota/iota-sdk/transactions";
import { getClockRef } from "../utils";

export function new_(didDoc: Uint8Array, packageId: string): Promise<Uint8Array> {
    const tx = new Transaction();
    const didDocArg = tx.pure.vector("u8", didDoc);
    const clock = getClockRef(tx);

    tx.moveCall({
        target: `${packageId}::identity::new`,
        arguments: [didDocArg, clock],
    });

    return tx.build();
}

export function newWithControllers(
    didDoc: Uint8Array,
    controllers: [string, number][],
    threshold: number,
    packageId: string,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const ids = tx.pure.vector("address", controllers.map(controller => controller[0]));
    const vps = tx.pure.vector("u64", controllers.map(controller => controller[1]));
    const controllersArg = tx.moveCall({
        target: `${packageId}::utils::vec_map_from_keys_values`,
        typeArguments: ["address", "u64"],
        arguments: [ids, vps],
    });
    const controllersThatCanDelegate = tx.moveCall({
        target: "0x2::vec_map::empty",
        typeArguments: ["address", "u64"],
        arguments: [],
    });
    const didDocArg = tx.pure.vector("u8", didDoc);
    const clock = getClockRef(tx);
    const thresholdArg = tx.pure.u64(threshold);

    tx.moveCall({
        target: `${packageId}::identity::new_with_controllers`,
        arguments: [didDocArg, controllersArg, controllersThatCanDelegate, thresholdArg, clock],
    });

    return tx.build();
}
