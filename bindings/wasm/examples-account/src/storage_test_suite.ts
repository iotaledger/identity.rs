// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Storage, StorageTestSuite } from '../../node/identity_wasm.js';

/**
 * This example demonstrates the usage of the Storage test suite for a custom implementation.
 */
export async function storageTestSuite(storageFactory: () => Promise<Storage>) {
    await StorageTestSuite.didCreateTest(await storageFactory());
    await StorageTestSuite.keyGenerateTest(await storageFactory());
    await StorageTestSuite.keyDeleteTest(await storageFactory());
    await StorageTestSuite.keyInsertTest(await storageFactory());
    await StorageTestSuite.didListTest(await storageFactory());
    await StorageTestSuite.keySignEd25519Test(await storageFactory());
    await StorageTestSuite.didPurgeTest(await storageFactory());
}
