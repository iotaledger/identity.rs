// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod asset_move_calls;
pub(crate) mod identity_move_calls;
pub(crate) mod migration_move_calls;

pub(crate) use asset_move_calls::AssetMoveCallsRustSdk as AssetMoveCallsAdapter;
pub(crate) use identity_move_calls::IdentityMoveCallsRustSdk as IdentityMoveCallsAdapter;
pub(crate) use migration_move_calls::MigrationMoveCallsRustSdk as MigrationMoveCallsAdapter;