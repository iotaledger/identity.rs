// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::vec::Vec;
use std::option::Option;
use std::convert::{AsRef, TryFrom};
use std::result::Result::Ok;
use std::option::Option::Some;
use std::str::FromStr;
use std::string::String;

use Result;

use rand::Rng;
use anyhow::anyhow;

use serde::{ser::Error, Deserialize, Serialize};
use serde_with::serde_as;

use fastcrypto::encoding::{Hex, Encoding, decode_bytes_hex};
use fastcrypto::hash::HashFunction;

use crate::ident_str;

use super::super::move_core_types::language_storage::{StructTag, TypeTag, ModuleId};
use super::super::move_core_types::identifier::IdentStr;
use super::super::move_core_types::account_address::AccountAddress;

use super::{IOTA_FRAMEWORK_ADDRESS, IOTA_CLOCK_OBJECT_ID, IOTA_SYSTEM_ADDRESS, MOVE_STDLIB_ADDRESS};
use super::balance::Balance;
use super::coin::{Coin, CoinMetadata, TreasuryCap, COIN_MODULE_NAME, COIN_STRUCT_NAME};
use super::crypto::{AuthorityPublicKeyBytes, IotaPublicKey, DefaultHash, PublicKey};
use super::dynamic_field::DynamicFieldInfo;
use super::error::{IotaError, IotaResult};
use super::gas_coin::GAS;
use super::governance::{StakedIota, STAKING_POOL_MODULE_NAME, STAKED_IOTA_STRUCT_NAME};
use super::iota_serde::{Readable, HexAccountAddress, parse_iota_struct_tag, to_iota_struct_tag_string};
use super::timelock::timelock::{self, TimeLock, TimelockedStakedIota};
use super::stardust::nft::Nft;
use super::gas_coin::GasCoin;
use super::object::{Owner};

pub use super::digests::{ObjectDigest, TransactionDigest};

pub type EpochId = u64;

// TODO: the stake and voting power of a validator can be different so
// in some places when we are actually referring to the voting power, we
// should use a different type alias, field name, etc.
pub type StakeUnit = u64;

pub type ObjectRef = (ObjectID, SequenceNumber, ObjectDigest);

pub type AuthorityName = AuthorityPublicKeyBytes;

pub type CommandIndex = usize;

/// Type parameters are encoded as indices. This index can also be used to
/// lookup the kind of a type parameter in the `FunctionHandle` and
/// `StructHandle`.
pub type TypeParameterIndex = u16;
pub type CodeOffset = u16;

#[derive(
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    Hash,
    Default,
    Debug,
    Serialize,
    Deserialize,
)]
pub struct SequenceNumber(u64);

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

// TODO: rename to version
impl SequenceNumber {
    pub const MIN: SequenceNumber = SequenceNumber(u64::MIN);
    pub const MAX: SequenceNumber = SequenceNumber(0x7fff_ffff_ffff_ffff);

    pub const fn new() -> Self {
        SequenceNumber(0)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn from_u64(u: u64) -> Self {
        SequenceNumber(u)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, thiserror::Error)]
pub enum ObjectIDParseError {
    #[error("ObjectID hex literal must start with 0x")]
    HexLiteralPrefixMissing,

    #[error("Could not convert from bytes slice")]
    TryFromSliceError,
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectID(
    #[serde_as(as = "Readable<HexAccountAddress, _>")]
    AccountAddress,
);

impl ObjectID {
    /// The number of bytes in an address.
    pub const LENGTH: usize = AccountAddress::LENGTH;
    /// Hex address: 0x0
    pub const ZERO: Self = Self::new([0u8; Self::LENGTH]);
    pub const MAX: Self = Self::new([0xff; Self::LENGTH]);
    /// Create a new ObjectID
    pub const fn new(obj_id: [u8; Self::LENGTH]) -> Self {
        Self(AccountAddress::new(obj_id))
    }

    /// Const fn variant of `<ObjectID as From<AccountAddress>>::from`
    pub const fn from_address(addr: AccountAddress) -> Self {
        Self(addr)
    }

    /// Return a random ObjectID.
    pub fn random() -> Self {
        Self::from(AccountAddress::random())
    }

    /// Return the underlying bytes buffer of the ObjectID.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Parse the ObjectID from byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ObjectIDParseError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| ObjectIDParseError::TryFromSliceError)
            .map(ObjectID::new)
    }

    /// Return the underlying bytes array of the ObjectID.
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0.into_bytes()
    }

    /// Make an ObjectID with padding 0s before the single byte.
    pub const fn from_single_byte(byte: u8) -> ObjectID {
        let mut bytes = [0u8; Self::LENGTH];
        bytes[Self::LENGTH - 1] = byte;
        ObjectID::new(bytes)
    }

    /// Convert from hex string to ObjectID where the string is prefixed with 0x
    /// Padding 0s if the string is too short.
    pub fn from_hex_literal(literal: &str) -> Result<Self, ObjectIDParseError> {
        if !literal.starts_with("0x") {
            return Err(ObjectIDParseError::HexLiteralPrefixMissing);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            Self::from_str(&hex_str)
        } else {
            Self::from_str(&literal[2..])
        }
    }


    /// Return the full hex string with 0x prefix without removing trailing 0s.
    /// Prefer this over [fn to_hex_literal] if the string needs to be fully
    /// preserved.
    pub fn to_hex_uncompressed(&self) -> String {
        format!("{self}")
    }

    pub fn is_clock(&self) -> bool {
        *self == IOTA_CLOCK_OBJECT_ID
    }
}

impl From<IotaAddress> for ObjectID {
    fn from(address: IotaAddress) -> ObjectID {
        let tmp: AccountAddress = address.into();
        tmp.into()
    }
}

impl From<AccountAddress> for ObjectID {
    fn from(address: AccountAddress) -> Self {
        Self(address)
    }
}

impl fmt::Display for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl fmt::Debug for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl AsRef<[u8]> for ObjectID {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl TryFrom<&[u8]> for ObjectID {
    type Error = ObjectIDParseError;

    /// Tries to convert the provided byte array into ObjectID.
    fn try_from(bytes: &[u8]) -> Result<ObjectID, ObjectIDParseError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for ObjectID {
    type Error = ObjectIDParseError;

    /// Tries to convert the provided byte buffer into ObjectID.
    fn try_from(bytes: Vec<u8>) -> Result<ObjectID, ObjectIDParseError> {
        Self::from_bytes(bytes)
    }
}

impl FromStr for ObjectID {
    type Err = ObjectIDParseError;

    /// Parse ObjectID from hex string with or without 0x prefix, pad with 0s if
    /// needed.
    fn from_str(s: &str) -> Result<Self, ObjectIDParseError> {
        decode_bytes_hex(s).or_else(|_| Self::from_hex_literal(s))
    }
}

impl std::ops::Deref for ObjectID {
    type Target = AccountAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around StructTag with a space-efficient representation for common
/// types like coins The StructTag for a gas coin is 84 bytes, so using 1 byte
/// instead is a win. The inner representation is private to prevent incorrectly
/// constructing an `Other` instead of one of the specialized variants, e.g.
/// `Other(GasCoin::type_())` instead of `GasCoin`
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MoveObjectType(MoveObjectType_);

/// Even though it is declared public, it is the "private", internal
/// representation for `MoveObjectType`
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Deserialize, Serialize, Hash)]
pub enum MoveObjectType_ {
    /// A type that is not `0x2::coin::Coin<T>`
    Other(StructTag),
    /// A IOTA coin (i.e., `0x2::coin::Coin<0x2::iota::IOTA>`)
    GasCoin,
    /// A record of a staked IOTA coin (i.e., `0x3::staking_pool::StakedIota`)
    StakedIota,
    /// A non-IOTA coin type (i.e., `0x2::coin::Coin<T> where T !=
    /// 0x2::iota::IOTA`)
    Coin(TypeTag),
    // NOTE: if adding a new type here, and there are existing on-chain objects of that
    // type with Other(_), that is ok, but you must hand-roll PartialEq/Eq/Ord/maybe Hash
    // to make sure the new type and Other(_) are interpreted consistently.
}

impl MoveObjectType {
    pub fn gas_coin() -> Self {
        Self(MoveObjectType_::GasCoin)
    }

    pub fn staked_iota() -> Self {
        Self(MoveObjectType_::StakedIota)
    }

    pub fn timelocked_iota_balance() -> Self {
        Self(MoveObjectType_::Other(TimeLock::<Balance>::type_(
            Balance::type_(GAS::type_().into()).into(),
        )))
    }

    pub fn timelocked_staked_iota() -> Self {
        Self(MoveObjectType_::Other(TimelockedStakedIota::type_()))
    }

    pub fn stardust_nft() -> Self {
        Self(MoveObjectType_::Other(Nft::tag()))
    }

    pub fn address(&self) -> AccountAddress {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) => IOTA_FRAMEWORK_ADDRESS,
            MoveObjectType_::StakedIota => IOTA_SYSTEM_ADDRESS,
            MoveObjectType_::Other(s) => s.address,
        }
    }

    pub fn module(&self) -> &IdentStr {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) => COIN_MODULE_NAME,
            MoveObjectType_::StakedIota => STAKING_POOL_MODULE_NAME,
            MoveObjectType_::Other(s) => &s.module,
        }
    }

    pub fn name(&self) -> &IdentStr {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) => COIN_STRUCT_NAME,
            MoveObjectType_::StakedIota => STAKED_IOTA_STRUCT_NAME,
            MoveObjectType_::Other(s) => &s.name,
        }
    }

    pub fn type_params(&self) -> Vec<TypeTag> {
        match &self.0 {
            MoveObjectType_::GasCoin => vec![GAS::type_tag()],
            MoveObjectType_::StakedIota => vec![],
            MoveObjectType_::Coin(inner) => vec![inner.clone()],
            MoveObjectType_::Other(s) => s.type_params.clone(),
        }
    }

    pub fn into_type_params(self) -> Vec<TypeTag> {
        match self.0 {
            MoveObjectType_::GasCoin => vec![GAS::type_tag()],
            MoveObjectType_::StakedIota => vec![],
            MoveObjectType_::Coin(inner) => vec![inner],
            MoveObjectType_::Other(s) => s.type_params,
        }
    }

    pub fn coin_type_maybe(&self) -> Option<TypeTag> {
        match &self.0 {
            MoveObjectType_::GasCoin => Some(GAS::type_tag()),
            MoveObjectType_::Coin(inner) => Some(inner.clone()),
            MoveObjectType_::StakedIota => None,
            MoveObjectType_::Other(_) => None,
        }
    }

    pub fn module_id(&self) -> ModuleId {
        ModuleId::new(self.address(), self.module().to_owned())
    }

    pub fn size_for_gas_metering(&self) -> usize {
        // unwraps safe because a `StructTag` cannot fail to serialize
        match &self.0 {
            MoveObjectType_::GasCoin => 1,
            MoveObjectType_::StakedIota => 1,
            MoveObjectType_::Coin(inner) => bcs::serialized_size(inner).unwrap() + 1,
            MoveObjectType_::Other(s) => bcs::serialized_size(s).unwrap() + 1,
        }
    }

    /// Return true if `self` is `0x2::coin::Coin<T>` for some T (note: T can be
    /// IOTA)
    pub fn is_coin(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) => true,
            MoveObjectType_::StakedIota | MoveObjectType_::Other(_) => false,
        }
    }

    /// Return true if `self` is 0x2::coin::Coin<0x2::iota::IOTA>
    pub fn is_gas_coin(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin => true,
            MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) | MoveObjectType_::Other(_) => {
                false
            }
        }
    }

    /// Return true if `self` is `0x2::coin::Coin<t>`
    pub fn is_coin_t(&self, t: &TypeTag) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin => GAS::is_gas_type(t),
            MoveObjectType_::Coin(c) => t == c,
            MoveObjectType_::StakedIota | MoveObjectType_::Other(_) => false,
        }
    }

    pub fn is_staked_iota(&self) -> bool {
        match &self.0 {
            MoveObjectType_::StakedIota => true,
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) | MoveObjectType_::Other(_) => {
                false
            }
        }
    }

    pub fn is_coin_metadata(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => CoinMetadata::is_coin_metadata(s),
        }
    }

    pub fn is_treasury_cap(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => TreasuryCap::is_treasury_type(s),
        }
    }

    pub fn is_upgrade_cap(&self) -> bool {
        self.address() == IOTA_FRAMEWORK_ADDRESS
            && self.module().as_str() == "package"
            && self.name().as_str() == "UpgradeCap"
    }

    pub fn is_regulated_coin_metadata(&self) -> bool {
        self.address() == IOTA_FRAMEWORK_ADDRESS
            && self.module().as_str() == "coin"
            && self.name().as_str() == "RegulatedCoinMetadata"
    }

    pub fn is_coin_deny_cap(&self) -> bool {
        self.address() == IOTA_FRAMEWORK_ADDRESS
            && self.module().as_str() == "coin"
            && self.name().as_str() == "DenyCap"
    }

    pub fn is_dynamic_field(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => DynamicFieldInfo::is_dynamic_field(s),
        }
    }

    pub fn is_timelock(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => timelock::is_timelock(s),
        }
    }

    pub fn is_timelocked_balance(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => timelock::is_timelocked_balance(s),
        }
    }

    pub fn is_timelocked_staked_iota(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                false
            }
            MoveObjectType_::Other(s) => TimelockedStakedIota::is_timelocked_staked_iota(s),
        }
    }

    pub fn try_extract_field_value(&self) -> IotaResult<TypeTag> {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) => {
                Err(IotaError::ObjectDeserialization {
                    error: "Error extracting dynamic object value from Coin object".to_string(),
                })
            }
            MoveObjectType_::Other(s) => DynamicFieldInfo::try_extract_field_value(s),
        }
    }
}

impl From<StructTag> for MoveObjectType {
    fn from(mut s: StructTag) -> Self {
        Self(if GasCoin::is_gas_coin(&s) {
            MoveObjectType_::GasCoin
        } else if Coin::is_coin(&s) {
            // unwrap safe because a coin has exactly one type parameter
            MoveObjectType_::Coin(s.type_params.pop().unwrap())
        } else if StakedIota::is_staked_iota(&s) {
            MoveObjectType_::StakedIota
        } else {
            MoveObjectType_::Other(s)
        })
    }
}

impl From<MoveObjectType> for StructTag {
    fn from(t: MoveObjectType) -> Self {
        match t.0 {
            MoveObjectType_::GasCoin => GasCoin::type_(),
            MoveObjectType_::StakedIota => StakedIota::type_(),
            MoveObjectType_::Coin(inner) => Coin::type_(inner),
            MoveObjectType_::Other(s) => s,
        }
    }
}

impl From<MoveObjectType> for TypeTag {
    fn from(o: MoveObjectType) -> TypeTag {
        let s: StructTag = o.into();
        TypeTag::Struct(Box::new(s))
    }
}

/// Type of a Iota object
#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ObjectType {
    /// Move package containing one or more bytecode modules
    Package,
    /// A Move struct of the given type
    Struct(MoveObjectType),
}

impl TryFrom<ObjectType> for StructTag {
    type Error = anyhow::Error;

    fn try_from(o: ObjectType) -> Result<Self, anyhow::Error> {
        match o {
            ObjectType::Package => Err(anyhow!("Cannot create StructTag from Package")),
            ObjectType::Struct(move_object_type) => Ok(move_object_type.into()),
        }
    }
}

impl FromStr for ObjectType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == PACKAGE {
            Ok(ObjectType::Package)
        } else {
            let tag = parse_iota_struct_tag(s)?;
            Ok(ObjectType::Struct(MoveObjectType::from(tag)))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ObjectInfo {
    pub object_id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
    pub type_: ObjectType,
    pub owner: Owner,
    pub previous_transaction: TransactionDigest,
}

const PACKAGE: &str = "package";
impl ObjectType {
    pub fn is_gas_coin(&self) -> bool {
        matches!(self, ObjectType::Struct(s) if s.is_gas_coin())
    }

    pub fn is_coin(&self) -> bool {
        matches!(self, ObjectType::Struct(s) if s.is_coin())
    }

    /// Return true if `self` is `0x2::coin::Coin<t>`
    pub fn is_coin_t(&self, t: &TypeTag) -> bool {
        matches!(self, ObjectType::Struct(s) if s.is_coin_t(t))
    }

    pub fn is_package(&self) -> bool {
        matches!(self, ObjectType::Package)
    }
}

impl From<ObjectInfo> for ObjectRef {
    fn from(info: ObjectInfo) -> Self {
        (info.object_id, info.version, info.digest)
    }
}

impl From<&ObjectInfo> for ObjectRef {
    fn from(info: &ObjectInfo) -> Self {
        (info.object_id, info.version, info.digest)
    }
}

pub const IOTA_ADDRESS_LENGTH: usize = ObjectID::LENGTH;

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct IotaAddress(
    #[serde_as(as = "Readable<Hex, _>")]
    [u8; IOTA_ADDRESS_LENGTH],
);

impl IotaAddress {
    pub const ZERO: Self = Self([0u8; IOTA_ADDRESS_LENGTH]);

    /// Convert the address to a byte buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn generate<R: rand::RngCore + rand::CryptoRng>(mut rng: R) -> Self {
        let buf: [u8; IOTA_ADDRESS_LENGTH] = rng.gen();
        Self(buf)
    }

    /// Serialize an `Option<IotaAddress>` in Hex.
    pub fn optional_address_as_hex<S>(
        key: &Option<IotaAddress>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
    {
        serializer.serialize_str(&key.map(Hex::encode).unwrap_or_default())
    }

    /// Deserialize into an `Option<IotaAddress>`.
    pub fn optional_address_from_hex<'de, D>(
        deserializer: D,
    ) -> Result<Option<IotaAddress>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = decode_bytes_hex(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(value))
    }

    /// Return the underlying byte array of a IotaAddress.
    pub fn to_inner(self) -> [u8; IOTA_ADDRESS_LENGTH] {
        self.0
    }

    /// Parse a IotaAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, IotaError> {
        <[u8; IOTA_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| IotaError::InvalidAddress)
            .map(IotaAddress)
    }
}

impl From<ObjectID> for IotaAddress {
    fn from(object_id: ObjectID) -> IotaAddress {
        Self(object_id.into_bytes())
    }
}

impl From<AccountAddress> for IotaAddress {
    fn from(address: AccountAddress) -> IotaAddress {
        Self(address.into_bytes())
    }
}

impl TryFrom<&[u8]> for IotaAddress {
    type Error = IotaError;

    /// Tries to convert the provided byte array into a IotaAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, IotaError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for IotaAddress {
    type Error = IotaError;

    /// Tries to convert the provided byte buffer into a IotaAddress.
    fn try_from(bytes: Vec<u8>) -> Result<Self, IotaError> {
        Self::from_bytes(bytes)
    }
}

impl AsRef<[u8]> for IotaAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl FromStr for IotaAddress {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode_bytes_hex(s).map_err(|e| anyhow!(e))
    }
}

impl<T: IotaPublicKey> From<&T> for IotaAddress {
    fn from(pk: &T) -> Self {
        let mut hasher = DefaultHash::default();
        T::SIGNATURE_SCHEME.update_hasher_with_flag(&mut hasher);
        hasher.update(pk);
        let g_arr = hasher.finalize();
        IotaAddress(g_arr.digest)
    }
}

impl From<&PublicKey> for IotaAddress {
    fn from(pk: &PublicKey) -> Self {
        let mut hasher = DefaultHash::default();
        pk.scheme().update_hasher_with_flag(&mut hasher);
        hasher.update(pk);
        let g_arr = hasher.finalize();
        IotaAddress(g_arr.digest)
    }
}

impl fmt::Display for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl fmt::Debug for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl fmt::Display for MoveObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let s: StructTag = self.clone().into();
        write!(
            f,
            "{}",
            to_iota_struct_tag_string(&s).map_err(fmt::Error::custom)?
        )
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Package => write!(f, "{}", PACKAGE),
            ObjectType::Struct(t) => write!(f, "{}", t),
        }
    }
}

impl From<ObjectID> for AccountAddress {
    fn from(obj_id: ObjectID) -> Self {
        obj_id.0
    }
}

impl From<IotaAddress> for AccountAddress {
    fn from(address: IotaAddress) -> Self {
        Self::new(address.0)
    }
}

pub const STD_OPTION_MODULE_NAME: &IdentStr = ident_str!("option");
pub const STD_OPTION_STRUCT_NAME: &IdentStr = ident_str!("Option");
pub const RESOLVED_STD_OPTION: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    STD_OPTION_MODULE_NAME,
    STD_OPTION_STRUCT_NAME,
);

pub const STD_ASCII_MODULE_NAME: &IdentStr = ident_str!("ascii");
pub const STD_ASCII_STRUCT_NAME: &IdentStr = ident_str!("String");
pub const RESOLVED_ASCII_STR: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    STD_ASCII_MODULE_NAME,
    STD_ASCII_STRUCT_NAME,
);

pub const STD_UTF8_MODULE_NAME: &IdentStr = ident_str!("string");
pub const STD_UTF8_STRUCT_NAME: &IdentStr = ident_str!("String");
pub const RESOLVED_UTF8_STR: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MOVE_STDLIB_ADDRESS,
    STD_UTF8_MODULE_NAME,
    STD_UTF8_STRUCT_NAME,
);