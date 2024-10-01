// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::ident_str;

use super::super::{
    move_core_types::{
        account_address::AccountAddress,
        identifier::IdentStr,
        language_storage::{StructTag, TypeTag},
        annotated_value::{MoveStructLayout, MoveFieldLayout, MoveTypeLayout},
    },
};

use super::{
    base_types::ObjectID,
    MoveTypeTagTrait,
    IOTA_FRAMEWORK_ADDRESS,
};

pub const OBJECT_MODULE_NAME_STR: &str = "object";
pub const OBJECT_MODULE_NAME: &IdentStr = ident_str!(OBJECT_MODULE_NAME_STR);
pub const UID_STRUCT_NAME: &IdentStr = ident_str!("UID");
pub const ID_STRUCT_NAME: &IdentStr = ident_str!("ID");
pub const RESOLVED_IOTA_ID: (&AccountAddress, &IdentStr, &IdentStr) =
    (&IOTA_FRAMEWORK_ADDRESS, OBJECT_MODULE_NAME, ID_STRUCT_NAME);

/// Rust version of the Move iota::object::Info type
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UID {
    pub id: ID,
}

/// Rust version of the Move iota::object::ID type
#[derive(Debug, Hash, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(transparent)]
pub struct ID {
    pub bytes: ObjectID,
}

impl UID {
    pub fn new(bytes: ObjectID) -> Self {
        Self {
            id: { ID::new(bytes) },
        }
    }

    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: OBJECT_MODULE_NAME.to_owned(),
            name: UID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn object_id(&self) -> &ObjectID {
        &self.id.bytes
    }

    pub fn to_bcs_bytes(&self) -> Vec<u8> {
        bcs::to_bytes(&self).unwrap()
    }

    pub fn layout() -> MoveStructLayout {
        MoveStructLayout {
            type_: Self::type_(),
            fields: vec![MoveFieldLayout::new(
                ident_str!("id").to_owned(),
                MoveTypeLayout::Struct(ID::layout()),
            )],
        }
    }
}

impl ID {
    pub fn new(object_id: ObjectID) -> Self {
        Self { bytes: object_id }
    }

    pub fn type_() -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: OBJECT_MODULE_NAME.to_owned(),
            name: ID_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }

    pub fn layout() -> MoveStructLayout {
        MoveStructLayout {
            type_: Self::type_(),
            fields: vec![MoveFieldLayout::new(
                ident_str!("bytes").to_owned(),
                MoveTypeLayout::Address,
            )],
        }
    }
}

impl MoveTypeTagTrait for ID {
    fn get_type_tag() -> TypeTag {
        TypeTag::Struct(Box::new(Self::type_()))
    }
}