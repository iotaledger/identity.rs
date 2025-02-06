// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_identity::did_deactivation_proposal {
    public struct DidDeactivation has store, copy, drop {}

    public fun new(): DidDeactivation {
        DidDeactivation {}
    }
}
