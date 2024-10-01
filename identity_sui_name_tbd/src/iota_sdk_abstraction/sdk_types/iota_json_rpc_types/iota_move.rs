// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::boxed::Box;
use std::fmt::{self, Display, Formatter, Write};

use itertools::Itertools;

use serde::Deserialize;
use serde::Serialize;
use serde_with::{serde_as};
use serde_json::{json, Value};

use tracing::warn;

use crate::iota_sdk_abstraction::types::{
    base_types::{IotaAddress, ObjectID},
    iota_serde::IotaStructTag,
};

use super::super::move_core_types::{
    language_storage::StructTag,
    annotated_value::{MoveStruct, MoveValue},
    identifier::Identifier,
};

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveValue")]
pub enum IotaMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(IotaAddress),
    Vector(Vec<IotaMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(IotaMoveStruct),
    Option(Box<Option<IotaMoveValue>>),
}

impl IotaMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            IotaMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            IotaMoveValue::Vector(values) => IotaMoveStruct::Runtime(values).to_json_value(),
            IotaMoveValue::Number(v) => json!(v),
            IotaMoveValue::Bool(v) => json!(v),
            IotaMoveValue::Address(v) => json!(v),
            IotaMoveValue::String(v) => json!(v),
            IotaMoveValue::UID { id } => json!({ "id": id }),
            IotaMoveValue::Option(v) => json!(v),
        }
    }
}

impl Display for IotaMoveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IotaMoveValue::Number(value) => write!(writer, "{value}")?,
            IotaMoveValue::Bool(value) => write!(writer, "{value}")?,
            IotaMoveValue::Address(value) => write!(writer, "{value}")?,
            IotaMoveValue::String(value) => write!(writer, "{value}")?,
            IotaMoveValue::UID { id } => write!(writer, "{id}")?,
            IotaMoveValue::Struct(value) => write!(writer, "{value}")?,
            IotaMoveValue::Option(value) => write!(writer, "{value:?}")?,
            IotaMoveValue::Vector(vec) => {
                write!(
                    writer,
                    "{}",
                    vec.iter().map(|value| format!("{value}")).join(",\n")
                )?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl From<MoveValue> for IotaMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => IotaMoveValue::Number(value.into()),
            MoveValue::U16(value) => IotaMoveValue::Number(value.into()),
            MoveValue::U32(value) => IotaMoveValue::Number(value),
            MoveValue::U64(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => IotaMoveValue::Bool(value),
            MoveValue::Vector(values) => {
                IotaMoveValue::Vector(values.into_iter().map(|value| value.into()).collect())
            }
            MoveValue::Struct(value) => {
                // Best effort Iota core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                IotaMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                IotaMoveValue::Address(IotaAddress::from(ObjectID::from(value)))
            }
        }
    }
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| {
                if let MoveValue::U8(u8) = value {
                    Some(*u8)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveStruct")]
pub enum IotaMoveStruct {
    Runtime(Vec<IotaMoveValue>),
    WithTypes {
        #[serde(rename = "type")]
        #[serde_as(as = "IotaStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, IotaMoveValue>,
    },
    WithFields(BTreeMap<String, IotaMoveValue>),
}

impl IotaMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            IotaMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the
            // client side.
            IotaMoveStruct::WithTypes { type_: _, fields } | IotaMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }

    pub fn read_dynamic_field_value(&self, field_name: &str) -> Option<IotaMoveValue> {
        match self {
            IotaMoveStruct::WithFields(fields) => fields.get(field_name).cloned(),
            IotaMoveStruct::WithTypes { type_: _, fields } => fields.get(field_name).cloned(),
            _ => None,
        }
    }
}

impl Display for IotaMoveStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = String::new();
        match self {
            IotaMoveStruct::Runtime(_) => {}
            IotaMoveStruct::WithFields(fields) => {
                for (name, value) in fields {
                    writeln!(writer, "{}: {value}", name)?;
                }
            }
            IotaMoveStruct::WithTypes { type_, fields } => {
                writeln!(writer)?;
                writeln!(writer, "  {}: {type_}", "type")?;
                for (name, value) in fields {
                    let value = format!("{value}");
                    let value = if value.starts_with('\n') {
                        indent(&value, 2)
                    } else {
                        value
                    };
                    writeln!(writer, "  {}: {value}", name)?;
                }
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

fn indent<T: Display>(d: &T, indent: usize) -> String {
    d.to_string()
        .lines()
        .map(|line| format!("{:indent$}{line}", ""))
        .join("\n")
}

fn try_convert_type(
    type_: &StructTag,
    fields: &[(Identifier, MoveValue)],
) -> Option<IotaMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(IotaMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(IotaMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(IotaMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(IotaMoveValue::from);
            if let Some(IotaMoveValue::Address(address)) = id {
                return Some(IotaMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(IotaMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(IotaMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(IotaMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to IotaMoveValue"
    );
    None
}

impl From<MoveStruct> for IotaMoveStruct {
    fn from(move_struct: MoveStruct) -> Self {
        IotaMoveStruct::WithTypes {
            type_: move_struct.type_,
            fields: move_struct
                .fields
                .into_iter()
                .map(|(id, value)| (id.into_string(), value.into()))
                .collect(),
        }
    }
}