// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::move_types::language_storage::TypeTag;
use crate::types::base_types::ObjectID;
use crate::MoveType;
use crate::TypedValue;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotaVerifiableCredential {
    data: Vec<u8>,
}

impl IotaVerifiableCredential {
    pub fn new(data: Vec<u8>) -> IotaVerifiableCredential {
        IotaVerifiableCredential { data }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl MoveType for IotaVerifiableCredential {
    fn move_type(package: ObjectID) -> TypeTag {
        TypeTag::from_str(&format!("{package}::public_vc::PublicVc")).expect("valid utf8")
    }

    fn get_typed_value(&self, _package: ObjectID) -> TypedValue<Self>
    where
        Self: MoveType,
        Self: Sized,
    {
        TypedValue::IotaVerifiableCredential(self)
    }
}
