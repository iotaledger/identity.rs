// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::ident_str;

use super::super::move_core_types::language_storage::{StructTag, TypeTag};
use super::super::move_core_types::identifier::IdentStr;
use super::super::move_core_types::annotated_value::{MoveStructLayout, MoveFieldLayout, MoveTypeLayout};

use super::id::UID;
use super::IOTA_FRAMEWORK_ADDRESS;
use super::error::{IotaError, ExecutionError, ExecutionErrorKind};
use super::balance::{Supply, Balance};
use super::base_types::ObjectID;

pub const COIN_MODULE_NAME: &IdentStr = ident_str!("coin");
pub const COIN_STRUCT_NAME: &IdentStr = ident_str!("Coin");
pub const COIN_METADATA_STRUCT_NAME: &IdentStr = ident_str!("CoinMetadata");
pub const COIN_TREASURE_CAP_NAME: &IdentStr = ident_str!("TreasuryCap");

pub const PAY_MODULE_NAME: &IdentStr = ident_str!("pay");
pub const PAY_JOIN_FUNC_NAME: &IdentStr = ident_str!("join");
pub const PAY_SPLIT_N_FUNC_NAME: &IdentStr = ident_str!("divide_and_keep");
pub const PAY_SPLIT_VEC_FUNC_NAME: &IdentStr = ident_str!("split_vec");

// Rust version of the Move iota::coin::Coin type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Coin {
    pub id: UID,
    pub balance: Balance,
}

impl Coin {
    pub fn new(id: UID, value: u64) -> Self {
        Self {
            id,
            balance: Balance::new(value),
        }
    }

    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            name: COIN_STRUCT_NAME.to_owned(),
            module: COIN_MODULE_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }

    /// Is this other StructTag representing a Coin?
    pub fn is_coin(other: &StructTag) -> bool {
        other.address == IOTA_FRAMEWORK_ADDRESS
            && other.module.as_ident_str() == COIN_MODULE_NAME
            && other.name.as_ident_str() == COIN_STRUCT_NAME
    }

    /// Create a coin from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, bcs::Error> {
        bcs::from_bytes(content)
    }

    pub fn id(&self) -> &ObjectID {
        self.id.object_id()
    }

    pub fn value(&self) -> u64 {
        self.balance.value()
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    pub fn layout(type_param: TypeTag) -> MoveStructLayout {
        MoveStructLayout {
            type_: Self::type_(type_param.clone()),
            fields: vec![
                MoveFieldLayout::new(
                    ident_str!("id").to_owned(),
                    MoveTypeLayout::Struct(UID::layout()),
                ),
                MoveFieldLayout::new(
                    ident_str!("balance").to_owned(),
                    MoveTypeLayout::Struct(Balance::layout(type_param)),
                ),
            ],
        }
    }

    /// Add balance to this coin, erroring if the new total balance exceeds the
    /// maximum
    pub fn add(&mut self, balance: Balance) -> Result<(), ExecutionError> {
        let Some(new_value) = self.value().checked_add(balance.value()) else {
            return Err(ExecutionError::from_kind(
                ExecutionErrorKind::CoinBalanceOverflow,
            ));
        };
        self.balance = Balance::new(new_value);
        Ok(())
    }

    // Split amount out of this coin to a new coin.
    // Related coin objects need to be updated in temporary_store to persist the
    // changes, including creating the coin object related to the newly created
    // coin.
    pub fn split(&mut self, amount: u64, new_coin_id: UID) -> Result<Coin, ExecutionError> {
        self.balance.withdraw(amount)?;
        Ok(Coin::new(new_coin_id, amount))
    }
}

// Rust version of the Move iota::coin::TreasuryCap type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TreasuryCap {
    pub id: UID,
    pub total_supply: Supply,
}

impl TreasuryCap {
    pub fn is_treasury_type(other: &StructTag) -> bool {
        other.address == IOTA_FRAMEWORK_ADDRESS
            && other.module.as_ident_str() == COIN_MODULE_NAME
            && other.name.as_ident_str() == COIN_TREASURE_CAP_NAME
    }

    /// Create a TreasuryCap from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, IotaError> {
        bcs::from_bytes(content).map_err(|err| IotaError::ObjectDeserialization {
            error: format!("Unable to deserialize TreasuryCap object: {}", err),
        })
    }

    pub fn type_(type_param: StructTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            name: COIN_TREASURE_CAP_NAME.to_owned(),
            module: COIN_MODULE_NAME.to_owned(),
            type_params: vec![TypeTag::Struct(Box::new(type_param))],
        }
    }
}

// Rust version of the Move iota::coin::CoinMetadata type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CoinMetadata {
    pub id: UID,
    /// Number of decimal places the coin uses.
    pub decimals: u8,
    /// Name for the token
    pub name: String,
    /// Symbol for the token
    pub symbol: String,
    /// Description of the token
    pub description: String,
    /// URL for the token logo
    pub icon_url: Option<String>,
}

impl CoinMetadata {
    /// Is this other StructTag representing a CoinMetadata?
    pub fn is_coin_metadata(other: &StructTag) -> bool {
        other.address == IOTA_FRAMEWORK_ADDRESS
            && other.module.as_ident_str() == COIN_MODULE_NAME
            && other.name.as_ident_str() == COIN_METADATA_STRUCT_NAME
    }

    /// Create a coin from BCS bytes
    pub fn from_bcs_bytes(content: &[u8]) -> Result<Self, IotaError> {
        bcs::from_bytes(content).map_err(|err| IotaError::ObjectDeserialization {
            error: format!("Unable to deserialize CoinMetadata object: {}", err),
        })
    }

    pub fn type_(type_param: StructTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            name: COIN_METADATA_STRUCT_NAME.to_owned(),
            module: COIN_MODULE_NAME.to_owned(),
            type_params: vec![TypeTag::Struct(Box::new(type_param))],
        }
    }
}