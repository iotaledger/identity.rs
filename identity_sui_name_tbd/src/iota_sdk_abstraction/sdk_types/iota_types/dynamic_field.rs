// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt,
    fmt::{Display, Formatter},
};

use fastcrypto::{encoding::Base58};

use crate::ident_str;

use super::super::move_core_types::{
    // annotated_value::{MoveStruct, MoveValue},
    identifier::IdentStr,
    language_storage::{StructTag, TypeTag},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

use super::{
    base_types::{ObjectID, SequenceNumber},
    digests::{ObjectDigest},
    error::{IotaError, IotaResult},
    id::UID,
    iota_serde::{IotaTypeTag, Readable},
    //object::Object,
    //storage::ObjectStore,
    IOTA_FRAMEWORK_ADDRESS,
};

const DYNAMIC_FIELD_MODULE_NAME: &IdentStr = ident_str!("dynamic_field");
const DYNAMIC_FIELD_FIELD_STRUCT_NAME: &IdentStr = ident_str!("Field");

const DYNAMIC_OBJECT_FIELD_MODULE_NAME: &IdentStr = ident_str!("dynamic_object_field");
const DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME: &IdentStr = ident_str!("Wrapper");

/// Rust version of the Move iota::dynamic_field::Field type
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Field<N, V> {
    pub id: UID,
    pub name: N,
    pub value: V,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFieldInfo {
    pub name: DynamicFieldName,
    #[serde_as(as = "Readable<Base58, _>")]
    pub bcs_name: Vec<u8>,
    pub type_: DynamicFieldType,
    pub object_type: String,
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFieldName {
    #[serde_as(as = "Readable<IotaTypeTag, _>")]
    pub type_: TypeTag,
    // Bincode does not like serde_json::Value, rocksdb will not insert the value without
    // serializing value as string. TODO: investigate if this can be removed after switch to
    // BCS.
    #[serde_as(as = "Readable<_, DisplayFromStr>")]
    pub value: Value,
}

impl Display for DynamicFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.type_, self.value)
    }
}

#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum DynamicFieldType {
    #[serde(rename_all = "camelCase")]
    DynamicField,
    DynamicObject,
}

impl Display for DynamicFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamicFieldType::DynamicField => write!(f, "DynamicField"),
            DynamicFieldType::DynamicObject => write!(f, "DynamicObject"),
        }
    }
}

impl DynamicFieldInfo {
    pub fn is_dynamic_field(tag: &StructTag) -> bool {
        tag.address == IOTA_FRAMEWORK_ADDRESS
            && tag.module.as_ident_str() == DYNAMIC_FIELD_MODULE_NAME
            && tag.name.as_ident_str() == DYNAMIC_FIELD_FIELD_STRUCT_NAME
    }

    pub fn is_dynamic_object_field_wrapper(tag: &StructTag) -> bool {
        tag.address == IOTA_FRAMEWORK_ADDRESS
            && tag.module.as_ident_str() == DYNAMIC_OBJECT_FIELD_MODULE_NAME
            && tag.name.as_ident_str() == DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME
    }

    pub fn dynamic_field_type(key: TypeTag, value: TypeTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            name: DYNAMIC_FIELD_FIELD_STRUCT_NAME.to_owned(),
            module: DYNAMIC_FIELD_MODULE_NAME.to_owned(),
            type_params: vec![key, value],
        }
    }

    pub fn dynamic_object_field_wrapper(key: TypeTag) -> StructTag {
        StructTag {
            address: IOTA_FRAMEWORK_ADDRESS,
            module: DYNAMIC_OBJECT_FIELD_MODULE_NAME.to_owned(),
            name: DYNAMIC_OBJECT_FIELD_WRAPPER_STRUCT_NAME.to_owned(),
            type_params: vec![key],
        }
    }

    pub fn try_extract_field_name(
        tag: &StructTag,
        type_: &DynamicFieldType,
    ) -> IotaResult<TypeTag> {
        match (type_, tag.type_params.first()) {
            (DynamicFieldType::DynamicField, Some(name_type)) => Ok(name_type.clone()),
            (DynamicFieldType::DynamicObject, Some(TypeTag::Struct(s))) => Ok(s
                .type_params
                .first()
                .ok_or_else(|| IotaError::ObjectDeserialization {
                    error: format!("Error extracting dynamic object name from object: {tag}"),
                })?
                .clone()),
            _ => Err(IotaError::ObjectDeserialization {
                error: format!("Error extracting dynamic object name from object: {tag}"),
            }),
        }
    }

    pub fn try_extract_field_value(tag: &StructTag) -> IotaResult<TypeTag> {
        match tag.type_params.last() {
            Some(value_type) => Ok(value_type.clone()),
            None => Err(IotaError::ObjectDeserialization {
                error: format!("Error extracting dynamic object value from object: {tag}"),
            }),
        }
    }
}