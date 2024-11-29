// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Predefined `AuthenticatedAsset`-related PTBs.
pub(crate) mod asset;
/// Predefined `OnChainIdentity`-related PTBs.
pub(crate) mod identity;
/// Predefined PTBs used to migrate a legacy Stardust's AliasOutput
/// to an `OnChainIdentity`.
pub(crate) mod migration;

mod utils;
