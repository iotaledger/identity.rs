// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::ident_str;

use super::super::super::move_core_types::{
    language_storage::{TypeTag, StructTag},
    identifier::{IdentStr},
};

use super::super::{
    IOTA_FRAMEWORK_ADDRESS,
    IOTA_SYSTEM_ADDRESS,
    base_types::{ObjectID, EpochId},
    balance::Balance,
    governance::StakedIota,
    id::UID,
};

use super::timelocked_staked_iota::{TIMELOCKED_STAKED_IOTA_MODULE_NAME, TIMELOCKED_STAKED_IOTA_STRUCT_NAME};

pub const TIMELOCK_MODULE_NAME: &IdentStr = ident_str!("timelock");
pub const TIMELOCK_STRUCT_NAME: &IdentStr = ident_str!("TimeLock");

/// Rust version of the Move stardust::TimeLock type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TimeLock<T> {
    id: UID,
    /// The locked object.
    locked: T,
    /// This is the epoch time stamp of when the lock expires.
    expiration_timestamp_ms: u64,
    /// Timelock related label.
    label: Option<String>,
}

impl<T> TimeLock<T> {
    /// Constructor.
    pub fn new(id: UID, locked: T, expiration_timestamp_ms: u64, label: Option<String>) -> Self {
        Self {
            id,
            locked,
            expiration_timestamp_ms,
            label,
        }
    }

    /// Get the TimeLock's `type`.
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: TIMELOCK_MODULE_NAME.to_owned(),
            name: TIMELOCK_STRUCT_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }

    /// Get the TimeLock's `id`.
    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    /// Get the TimeLock's `locked` object.
    pub fn locked(&self) -> &T {
        &self.locked
    }

    /// Get the TimeLock's `expiration_timestamp_ms`.
    pub fn expiration_timestamp_ms(&self) -> u64 {
        self.expiration_timestamp_ms
    }

    /// Get the TimeLock's `label``.
    pub fn label(&self) -> &Option<String> {
        &self.label
    }
}

impl<'de, T> TimeLock<T>
    where
        T: Serialize + Deserialize<'de>,
{
    /// Create a `TimeLock` from BCS bytes.
    pub fn from_bcs_bytes(content: &'de [u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    /// Serialize a `TimeLock` as a `Vec<u8>` of BCS.
    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }
}

/// Is this other StructTag representing a TimeLock?
pub fn is_timelock(other: &StructTag) -> bool {
    other.address == IOTA_FRAMEWORK_ADDRESS
        && other.module.as_ident_str() == TIMELOCK_MODULE_NAME
        && other.name.as_ident_str() == TIMELOCK_STRUCT_NAME
}

/// Is this other StructTag representing a `TimeLock<Balance<T>>`?
pub fn is_timelocked_balance(other: &StructTag) -> bool {
    if !is_timelock(other) {
        return false;
    }

    if other.type_params.len() != 1 {
        return false;
    }

    match &other.type_params[0] {
        TypeTag::Struct(tag) => Balance::is_balance(tag),
        _ => false,
    }
}

/// Rust version of the Move
/// stardust::timelocked_staked_iota::TimelockedStakedIota type.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TimelockedStakedIota {
    id: UID,
    /// A self-custodial object holding the staked IOTA tokens.
    staked_iota: StakedIota,
    /// This is the epoch time stamp of when the lock expires.
    expiration_timestamp_ms: u64,
    /// Timelock related label.
    label: Option<String>,
}

impl TimelockedStakedIota {
    /// Get the TimeLock's `type`.
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_SYSTEM_ADDRESS,
            module: TIMELOCKED_STAKED_IOTA_MODULE_NAME.to_owned(),
            name: TIMELOCKED_STAKED_IOTA_STRUCT_NAME.to_owned(),
            type_params: vec![],
        }
    }

    /// Is this other StructTag representing a TimelockedStakedIota?
    pub fn is_timelocked_staked_iota(s: &StructTag) -> bool {
        s.address == IOTA_SYSTEM_ADDRESS
            && s.module.as_ident_str() == TIMELOCKED_STAKED_IOTA_MODULE_NAME
            && s.name.as_ident_str() == TIMELOCKED_STAKED_IOTA_STRUCT_NAME
            && s.type_params.is_empty()
    }

    /// Get the TimelockedStakedIota's `id`.
    pub fn id(&self) -> ObjectID {
        self.id.id.bytes
    }

    /// Get the wrapped StakedIota's `pool_id`.
    pub fn pool_id(&self) -> ObjectID {
        self.staked_iota.pool_id()
    }

    /// Get the wrapped StakedIota's `activation_epoch`.
    pub fn activation_epoch(&self) -> EpochId {
        self.staked_iota.activation_epoch()
    }

    /// Get the wrapped StakedIota's `request_epoch`.
    pub fn request_epoch(&self) -> EpochId {
        // TODO: this might change when we implement warm up period.
        self.staked_iota.activation_epoch().saturating_sub(1)
    }

    /// Get the wrapped StakedIota's `principal`.
    pub fn principal(&self) -> u64 {
        self.staked_iota.principal()
    }

    /// Get the TimelockedStakedIota's `expiration_timestamp_ms`.
    pub fn expiration_timestamp_ms(&self) -> u64 {
        self.expiration_timestamp_ms
    }

    /// Get the TimelockedStakedIota's `label``.
    pub fn label(&self) -> &Option<String> {
        &self.label
    }
}