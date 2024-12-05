// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as, Bytes};

use super::base_types::{ObjectID, SequenceNumber};

/// Identifies a struct and the module it was defined in
#[derive(
Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, Hash)]
pub struct TypeOrigin {
    pub module_name: String,
    pub struct_name: String,
    pub package: ObjectID,
}

/// Upgraded package info for the linkage table
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct UpgradeInfo {
    /// ID of the upgraded packages
    pub upgraded_id: ObjectID,
    /// Version of the upgraded package
    pub upgraded_version: SequenceNumber,
}

// serde_bytes::ByteBuf is an analog of Vec<u8> with built-in fast
// serialization.
#[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MovePackage {
    id: ObjectID,
    /// Most move packages are uniquely identified by their ID (i.e. there is
    /// only one version per ID), but the version is still stored because
    /// one package may be an upgrade of another (at a different ID), in
    /// which case its version will be one greater than the version of the
    /// upgraded package.
    ///
    /// Framework packages are an exception to this rule -- all versions of the
    /// framework packages exist at the same ID, at increasing versions.
    ///
    /// In all cases, packages are referred to by move calls using just their
    /// ID, and they are always loaded at their latest version.
    version: SequenceNumber,
    // TODO use session cache
    #[serde_as(as = "BTreeMap<_, Bytes>")]
    module_map: BTreeMap<String, Vec<u8>>,

    /// Maps struct/module to a package version where it was first defined,
    /// stored as a vector for simple serialization and deserialization.
    type_origin_table: Vec<TypeOrigin>,

    // For each dependency, maps original package ID to the info about the (upgraded) dependency
    // version that this package is using
    linkage_table: BTreeMap<ObjectID, UpgradeInfo>,
}

impl MovePackage {
    pub fn id(&self) -> ObjectID {
        self.id
    }

    pub fn version(&self) -> SequenceNumber {
        self.version
    }

    pub fn serialized_module_map(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.module_map
    }

    pub fn type_origin_table(&self) -> &Vec<TypeOrigin> {
        &self.type_origin_table
    }

    pub fn linkage_table(&self) -> &BTreeMap<ObjectID, UpgradeInfo> {
        &self.linkage_table
    }
}
