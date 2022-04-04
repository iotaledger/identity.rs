// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { storageDidCreateTest } from '../../node/identity_wasm.js';
import { MemStore } from './memory_storage';

async function storageTest() {

    const storage = new MemStore();
    await storageDidCreateTest(storage);

}

export { storageTest }
