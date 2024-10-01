// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt,
    fmt::{Debug, Display, Formatter, Write},
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};
use std::marker::Sized;
use std::string::{String, ToString};
use std::result::Result::Ok;
use std::option::Option;
use std::option::Option::Some;

use fastcrypto::encoding::Hex;
use serde::{
    self,
    de::{Deserializer, Error},
    ser::{Error as SerError, Serializer},
    Deserialize, Serialize,
};
use serde_with::{serde_as, DeserializeAs, DisplayFromStr, SerializeAs};

use Result;

use super::super::move_core_types::{
    account_address::AccountAddress,
    language_storage::{StructTag, TypeTag}
};
// use super::address::ParsedAddress;
use super::{IOTA_FRAMEWORK_ADDRESS, DEEPBOOK_ADDRESS, MOVE_STDLIB_ADDRESS, IOTA_SYSTEM_ADDRESS,
            STARDUST_ADDRESS, IOTA_SYSTEM_STATE_ADDRESS, IOTA_CLOCK_ADDRESS};

/// The minimum and maximum protocol versions supported by this build.
const MIN_PROTOCOL_VERSION: u64 = 1;
pub const MAX_PROTOCOL_VERSION: u64 = 1;

/// Resolve well-known named addresses into numeric addresses.
pub fn resolve_address(addr: &str) -> Option<AccountAddress> {
    match addr {
        "deepbook" => Some(DEEPBOOK_ADDRESS),
        "std" => Some(MOVE_STDLIB_ADDRESS),
        "iota" => Some(IOTA_FRAMEWORK_ADDRESS),
        "iota_system" => Some(IOTA_SYSTEM_ADDRESS),
        "stardust" => Some(STARDUST_ADDRESS),
        _ => None,
    }
}

/// Parse `s` as a struct type: A fully-qualified name, optionally followed by a
/// list of type parameters (types -- see `parse_iota_type_tag`, separated by
/// commas, surrounded by angle brackets). Parsing succeeds if and only if `s`
/// matches this format exactly, with no remaining input. This function is
/// intended for use within the authority codebase.
pub fn parse_iota_struct_tag(s: &str) -> anyhow::Result<StructTag> {
    use super::super::move_command_line_common::types::ParsedStructType;
    ParsedStructType::parse(s)?.into_struct_tag(&resolve_address)
}

/// Parse `s` as a type: Either a struct type (see `parse_iota_struct_tag`), a
/// primitive type, or a vector with a type parameter. Parsing succeeds if and
/// only if `s` matches this format exactly, with no remaining input. This
/// function is intended for use within the authority codebase.
pub fn parse_iota_type_tag(s: &str) -> anyhow::Result<TypeTag> {
    use super::super::move_command_line_common::types::ParsedType;
    ParsedType::parse(s)?.into_type_tag(&resolve_address)
}

#[inline]
fn to_custom_error<'de, D, E>(e: E) -> D::Error
    where
        E: Debug,
        D: Deserializer<'de>,
{
    Error::custom(format!("byte deserialization failed, cause by: {:?}", e))
}

/// Use with serde_as to control serde for human-readable serialization and
/// deserialization `H` : serde_as SerializeAs/DeserializeAs delegation for
/// human readable in/output `R` : serde_as SerializeAs/DeserializeAs delegation
/// for non-human readable in/output
///
/// # Example:
///
/// ```text
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Example(#[serde_as(as = "Readable<DisplayFromStr, _>")] [u8; 20]);
/// ```
///
/// The above example will delegate human-readable serde to `DisplayFromStr`
/// and array tuple (default) for non-human-readable serializer.
pub struct Readable<H, R> {
    human_readable: PhantomData<H>,
    non_human_readable: PhantomData<R>,
}

impl<T: ?Sized, H, R> SerializeAs<T> for Readable<H, R>
    where
        H: SerializeAs<T>,
        R: SerializeAs<T>,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if serializer.is_human_readable() {
            H::serialize_as(value, serializer)
        } else {
            R::serialize_as(value, serializer)
        }
    }
}

impl<'de, R, H, T> DeserializeAs<'de, T> for Readable<H, R>
    where
        H: DeserializeAs<'de, T>,
        R: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            H::deserialize_as(deserializer)
        } else {
            R::deserialize_as(deserializer)
        }
    }
}

/// custom serde for AccountAddress
pub struct HexAccountAddress;

impl SerializeAs<AccountAddress> for HexAccountAddress {
    fn serialize_as<S>(value: &AccountAddress, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        Hex::serialize_as(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, AccountAddress> for HexAccountAddress {
    fn deserialize_as<D>(deserializer: D) -> Result<AccountAddress, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("0x") {
            AccountAddress::from_hex_literal(&s)
        } else {
            AccountAddress::from_hex(&s)
        }
            .map_err(to_custom_error::<'de, D, _>)
    }
}

/// Serializes a bitmap according to the roaring bitmap on-disk standard.
/// <https://github.com/RoaringBitmap/RoaringFormatSpec>
pub struct IotaBitmap;

pub struct IotaStructTag;

impl SerializeAs<StructTag> for IotaStructTag {
    fn serialize_as<S>(value: &StructTag, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let f = to_iota_struct_tag_string(value).map_err(S::Error::custom)?;
        f.serialize(serializer)
    }
}

const IOTA_ADDRESSES: [AccountAddress; 8] = [
    AccountAddress::ZERO,
    AccountAddress::ONE,
    IOTA_FRAMEWORK_ADDRESS,
    IOTA_SYSTEM_ADDRESS,
    STARDUST_ADDRESS,
    DEEPBOOK_ADDRESS,
    IOTA_SYSTEM_STATE_ADDRESS,
    IOTA_CLOCK_ADDRESS,
];
/// Serialize StructTag as a string, retaining the leading zeros in the address.
pub fn to_iota_struct_tag_string(value: &StructTag) -> Result<String, fmt::Error> {
    let mut f = String::new();
    // trim leading zeros if address is in IOTA_ADDRESSES
    let address = if IOTA_ADDRESSES.contains(&value.address) {
        value.address.short_str_lossless()
    } else {
        value.address.to_canonical_string(/* with_prefix */ false)
    };

    write!(f, "0x{}::{}::{}", address, value.module, value.name)?;
    if let Some(first_ty) = value.type_params.first() {
        write!(f, "<")?;
        write!(f, "{}", to_iota_type_tag_string(first_ty)?)?;
        for ty in value.type_params.iter().skip(1) {
            write!(f, ", {}", to_iota_type_tag_string(ty)?)?;
        }
        write!(f, ">")?;
    }
    Ok(f)
}

fn to_iota_type_tag_string(value: &TypeTag) -> Result<String, fmt::Error> {
    match value {
        TypeTag::Vector(t) => Ok(format!("vector<{}>", to_iota_type_tag_string(t)?)),
        TypeTag::Struct(s) => to_iota_struct_tag_string(s),
        _ => Ok(value.to_string()),
    }
}

impl<'de> DeserializeAs<'de, StructTag> for IotaStructTag {
    fn deserialize_as<D>(deserializer: D) -> Result<StructTag, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_struct_tag(&s).map_err(D::Error::custom)
    }
}

pub struct IotaTypeTag;

impl SerializeAs<TypeTag> for IotaTypeTag {
    fn serialize_as<S>(value: &TypeTag, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = to_iota_type_tag_string(value).map_err(S::Error::custom)?;
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, TypeTag> for IotaTypeTag {
    fn deserialize_as<D>(deserializer: D) -> Result<TypeTag, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_type_tag(&s).map_err(D::Error::custom)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct BigInt<T>(
    #[serde_as(as = "DisplayFromStr")]
    T,
)
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display;

impl<T> BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> SerializeAs<T> for BigInt<T>
    where
        T: Display + FromStr + Copy,
        <T as FromStr>::Err: Display,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        BigInt(*value).serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for BigInt<T>
    where
        T: Display + FromStr + Copy,
        <T as FromStr>::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(*BigInt::deserialize(deserializer)?)
    }
}

impl<T> From<T> for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    fn from(v: T) -> BigInt<T> {
        BigInt(v)
    }
}

impl<T> Deref for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Display for BigInt<T>
    where
        T: Display + FromStr,
        <T as FromStr>::Err: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct SequenceNumber(u64);

impl SerializeAs<super::base_types::SequenceNumber> for SequenceNumber {
    fn serialize_as<S>(
        value: &super::base_types::SequenceNumber,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = value.value().to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, super::base_types::SequenceNumber> for SequenceNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<super::base_types::SequenceNumber, D::Error>
        where
            D: Deserializer<'de>,
    {
        let b = BigInt::deserialize(deserializer)?;
        Ok(super::base_types::SequenceNumber::from_u64(*b))
    }
}

// Record history of protocol version allocations here:
//
// Version 1: Original version.
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtocolVersion(u64);

impl ProtocolVersion {
    // The minimum and maximum protocol version supported by this binary.
    // Counterintuitively, this constant may change over time as support for old
    // protocol versions is removed from the source. This ensures that when a
    // new network (such as a testnet) is created, its genesis committee will
    // use a protocol version that is actually supported by the binary.
    pub const MIN: Self = Self(MIN_PROTOCOL_VERSION);

    pub const MAX: Self = Self(MAX_PROTOCOL_VERSION);

    pub fn new(v: u64) -> Self {
        Self(v)
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    // For serde deserialization - we don't define a Default impl because there
    // isn't a single universally appropriate default value.
    pub fn max() -> Self {
        Self::MAX
    }
}

impl From<u64> for ProtocolVersion {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
#[serde(rename = "ProtocolVersion")]
pub struct AsProtocolVersion(u64);

impl SerializeAs<ProtocolVersion> for AsProtocolVersion {
    fn serialize_as<S>(value: &ProtocolVersion, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = value.as_u64().to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, ProtocolVersion> for AsProtocolVersion {
    fn deserialize_as<D>(deserializer: D) -> Result<ProtocolVersion, D::Error>
        where
            D: Deserializer<'de>,
    {
        let b = BigInt::<u64>::deserialize(deserializer)?;
        Ok(ProtocolVersion::from(*b))
    }
}