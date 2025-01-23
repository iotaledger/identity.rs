// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from "@iota/iota.js/dist/cjs/bcs/types";
import { ObjectRef, Transaction } from "@iota/iota.js/transactions";
import { getClockRef } from "./utils";

export function migrateDidOutput(
    didOutput: ObjectRef,
    migrationRegistry: SharedObjectRef,
    packageId: string,
    creationTimestamp?: number,
): Promise<Uint8Array> {
    const tx = new Transaction();
    const did_output = tx.objectRef(didOutput);
    const migration_registry = tx.sharedObjectRef(migrationRegistry);
    const clock = getClockRef(tx);
    let timestamp;
    if (creationTimestamp) {
        timestamp = tx.pure.u64(creationTimestamp);
    } else {
        timestamp = tx.moveCall({
            target: "0x2::clock::timestamp_ms",
            arguments: [clock],
        });
    }

    tx.moveCall({
        target: `${packageId}::migration::migrate_alias_output`,
        arguments: [did_output, migration_registry, timestamp, clock],
    });

    return tx.build();
}
