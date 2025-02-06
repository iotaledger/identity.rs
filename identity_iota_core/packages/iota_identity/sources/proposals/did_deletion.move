// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

module iota_identity::did_deletion_proposal {
    public struct DidDeletion has store, copy, drop {}

    public fun new(): DidDeletion {
        DidDeletion {}
    }
}

