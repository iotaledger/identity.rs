// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { useIotaKeytoolSigner } from "../1_advanced/1_keytool_signer";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("keytool_signer", async () => {
        await useIotaKeytoolSigner();
    });
});
