// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::ident_str;

use super::super::move_core_types::language_storage::{StructTag, TypeTag};
use super::super::move_core_types::identifier::IdentStr;
use super::super::move_core_types::annotated_value::MoveStructLayout;

use super::super::types::IOTA_FRAMEWORK_ADDRESS;

use super::coin::{Coin, TreasuryCap};
use super::base_types::{ObjectID};
use super::id::UID;
use super::balance::{Balance, Supply};
use std::fmt::{Display, Formatter};

/// The number of Nanos per Iota token
pub const NANOS_PER_IOTA: u64 = 1_000_000_000;

/// Total supply in IOTA at genesis, after the migration from a Stardust ledger,
/// before any inflation mechanism
pub const STARDUST_TOTAL_SUPPLY_IOTA: u64 = 4_600_000_000;

// Note: cannot use checked arithmetic here since `const unwrap` is still
// unstable.
/// Total supply at genesis denominated in Nanos, after the migration from a
/// Stardust ledger, before any inflation mechanism
pub const STARDUST_TOTAL_SUPPLY_NANOS: u64 = STARDUST_TOTAL_SUPPLY_IOTA * NANOS_PER_IOTA;

pub const GAS_MODULE_NAME: &IdentStr = ident_str!("iota");
pub const GAS_STRUCT_NAME: &IdentStr = ident_str!("IOTA");
pub const GAS_TREASURY_CAP_STRUCT_NAME: &IdentStr = ident_str!("IotaTreasuryCap");

pub struct GAS {}
impl GAS {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            name: GAS_STRUCT_NAME.to_owned(),
            module: GAS_MODULE_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(Self::type_()))
    }

    pub fn is_gas(other: &StructTag) -> bool {
        &Self::type_() == other
    }

    pub fn is_gas_type(other: &TypeTag) -> bool {
        match other {
            TypeTag::Struct(s) => Self::is_gas(s),
            _ => false,
        }
    }
}

/// Rust version of the Move iota::coin::Coin<Iota::iota::IOTA> type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GasCoin(pub Coin);

impl GasCoin {
    pub fn new(id: ObjectID, value: u64) -> Self {
        Self(Coin::new(UID::new(id), value))
    }

    pub fn value(&self) -> u64 {
        self.0.value()
    }

    pub fn type_() -> StructTag {
        Coin::type_(TypeTag::Struct(Box::new(GAS::type_())))
    }

    /// Return `true` if `s` is the type of a gas coin (i.e.,
    /// 0x2::coin::Coin<0x2::iota::IOTA>)
    pub fn is_gas_coin(s: &StructTag) -> bool {
        Coin::is_coin(s) && s.type_params.len() == 1 && GAS::is_gas_type(&s.type_params[0])
    }

    /// Return `true` if `s` is the type of a gas balance (i.e.,
    /// 0x2::balance::Balance<0x2::iota::IOTA>)
    pub fn is_gas_balance(s: &StructTag) -> bool {
        Balance::is_balance(s)
            && s.type_params.len() == 1
            && GAS::is_gas_type(&s.type_params[0])
    }

    pub fn id(&self) -> &ObjectID {
        self.0.id()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    pub fn layout() -> MoveStructLayout {
        Coin::layout(TypeTag::Struct(Box::new(GAS::type_())))
    }
}

impl Display for GasCoin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coin {{ id: {}, value: {} }}", self.id(), self.value())
    }
}

// Rust version of the IotaTreasuryCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct IotaTreasuryCap {
    pub inner: TreasuryCap,
}

impl IotaTreasuryCap {
    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: GAS_MODULE_NAME.to_owned(),
            name: GAS_TREASURY_CAP_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    /// Returns the total `Supply` of `Coin<IOTA>`.
    pub fn total_supply(&self) -> &Supply {
        &self.inner.total_supply
    }
}