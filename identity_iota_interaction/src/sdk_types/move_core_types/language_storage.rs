// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use super::{
    account_address::AccountAddress,
    identifier::{IdentStr, Identifier},
    parsing::types::{ParsedModuleId, ParsedStructType, ParsedType},
};

pub const CODE_TAG: u8 = 0;
pub const RESOURCE_TAG: u8 = 1;

/// Hex address: 0x1
pub const CORE_CODE_ADDRESS: AccountAddress = AccountAddress::ONE;

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub enum TypeTag {
    // alias for compatibility with old json serialized data.
    #[serde(rename = "bool", alias = "Bool")]
    Bool,
    #[serde(rename = "u8", alias = "U8")]
    U8,
    #[serde(rename = "u64", alias = "U64")]
    U64,
    #[serde(rename = "u128", alias = "U128")]
    U128,
    #[serde(rename = "address", alias = "Address")]
    Address,
    #[serde(rename = "signer", alias = "Signer")]
    Signer,
    #[serde(rename = "vector", alias = "Vector")]
    Vector(Box<TypeTag>),
    #[serde(rename = "struct", alias = "Struct")]
    Struct(Box<StructTag>),

    // NOTE: Added in bytecode version v6, do not reorder!
    #[serde(rename = "u16", alias = "U16")]
    U16,
    #[serde(rename = "u32", alias = "U32")]
    U32,
    #[serde(rename = "u256", alias = "U256")]
    U256,
}

impl TypeTag {
    /// Return a canonical string representation of the type. All types are
    /// represented using their source syntax:
    ///
    /// - "bool", "u8", "u16", "u32", "u64", "u128", "u256", "address",
    ///   "signer", "vector" for ground types.
    ///
    /// - Structs are represented as fully qualified type names, with or without
    ///   the prefix "0x" depending on the `with_prefix` flag, e.g.
    ///   `0x000...0001::string::String` or
    ///   `0x000...000a::m::T<0x000...000a::n::U<u64>>`.
    ///
    /// - Addresses are hex-encoded lowercase values of length 32 (zero-padded).
    ///
    /// Note: this function is guaranteed to be stable -- suitable for use
    /// inside Move native functions or the VM. By contrast, this type's
    /// `Display` implementation is subject to change and should be used
    /// inside code that needs to return a stable output (e.g. that might be
    /// committed to effects on-chain).
    pub fn to_canonical_string(&self, with_prefix: bool) -> String {
        self.to_canonical_display(with_prefix).to_string()
    }

    /// Implements the canonical string representation of the type with optional
    /// prefix 0x
    pub fn to_canonical_display(&self, with_prefix: bool) -> impl std::fmt::Display + '_ {
        struct CanonicalDisplay<'a> {
            data: &'a TypeTag,
            with_prefix: bool,
        }

        impl std::fmt::Display for CanonicalDisplay<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self.data {
                    TypeTag::Bool => write!(f, "bool"),
                    TypeTag::U8 => write!(f, "u8"),
                    TypeTag::U16 => write!(f, "u16"),
                    TypeTag::U32 => write!(f, "u32"),
                    TypeTag::U64 => write!(f, "u64"),
                    TypeTag::U128 => write!(f, "u128"),
                    TypeTag::U256 => write!(f, "u256"),
                    TypeTag::Address => write!(f, "address"),
                    TypeTag::Signer => write!(f, "signer"),
                    TypeTag::Vector(t) => {
                        write!(f, "vector<{}>", t.to_canonical_display(self.with_prefix))
                    }
                    TypeTag::Struct(s) => write!(f, "{}", s.to_canonical_display(self.with_prefix)),
                }
            }
        }

        CanonicalDisplay {
            data: self,
            with_prefix,
        }
    }
}

impl FromStr for TypeTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParsedType::parse(s)?.into_type_tag(&|_| None)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct StructTag {
    pub address: AccountAddress,
    pub module: Identifier,
    pub name: Identifier,
    // alias for compatibility with old json serialized data.
    #[serde(rename = "type_args", alias = "type_params")]
    pub type_params: Vec<TypeTag>,
}

impl StructTag {
    pub fn access_vector(&self) -> Vec<u8> {
        let mut key = vec![RESOURCE_TAG];
        key.append(&mut bcs::to_bytes(self).unwrap());
        key
    }

    /// Returns true if this is a `StructTag` for an `std::ascii::String` struct
    /// defined in the standard library at address `move_std_addr`.
    pub fn is_ascii_string(&self, move_std_addr: &AccountAddress) -> bool {
        self.address == *move_std_addr
            && self.module.as_str().eq("ascii")
            && self.name.as_str().eq("String")
    }

    /// Returns true if this is a `StructTag` for an `std::string::String`
    /// struct defined in the standard library at address `move_std_addr`.
    pub fn is_std_string(&self, move_std_addr: &AccountAddress) -> bool {
        self.address == *move_std_addr
            && self.module.as_str().eq("string")
            && self.name.as_str().eq("String")
    }

    pub fn module_id(&self) -> ModuleId {
        ModuleId::new(self.address, self.module.to_owned())
    }

    /// Return a canonical string representation of the struct.
    ///
    /// - Structs are represented as fully qualified type names, with or without
    ///   the prefix "0x" depending on the `with_prefix` flag, e.g.
    ///   `0x000...0001::string::String` or
    ///   `0x000...000a::m::T<0x000...000a::n::U<u64>>`.
    ///
    /// - Addresses are hex-encoded lowercase values of length 32 (zero-padded).
    ///
    /// Note: this function is guaranteed to be stable -- suitable for use
    /// inside Move native functions or the VM. By contrast, this type's
    /// `Display` implementation is subject to change and should be used
    /// inside code that needs to return a stable output (e.g. that might be
    /// committed to effects on-chain).
    pub fn to_canonical_string(&self, with_prefix: bool) -> String {
        self.to_canonical_display(with_prefix).to_string()
    }

    /// Implements the canonical string representation of the StructTag with
    /// optional prefix 0x
    pub fn to_canonical_display(&self, with_prefix: bool) -> impl std::fmt::Display + '_ {
        struct CanonicalDisplay<'a> {
            data: &'a StructTag,
            with_prefix: bool,
        }

        impl std::fmt::Display for CanonicalDisplay<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}::{}::{}",
                    self.data.address.to_canonical_display(self.with_prefix),
                    self.data.module,
                    self.data.name
                )?;

                if let Some(first_ty) = self.data.type_params.first() {
                    write!(f, "<")?;
                    write!(f, "{}", first_ty.to_canonical_display(self.with_prefix))?;
                    for ty in self.data.type_params.iter().skip(1) {
                        // Note that unlike Display for StructTag, there is no space between the
                        // comma and canonical display. This follows the
                        // original to_canonical_string() implementation.
                        write!(f, ",{}", ty.to_canonical_display(self.with_prefix))?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
        }

        CanonicalDisplay {
            data: self,
            with_prefix,
        }
    }
}

impl FromStr for StructTag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParsedStructType::parse(s)?.into_struct_tag(&|_| None)
    }
}

/// Represents the initial key into global storage where we first index by the
/// address, and then the struct tag
#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub struct ModuleId {
    address: AccountAddress,
    name: Identifier,
}

impl From<ModuleId> for (AccountAddress, Identifier) {
    fn from(module_id: ModuleId) -> Self {
        (module_id.address, module_id.name)
    }
}

impl ModuleId {
    pub fn new(address: AccountAddress, name: Identifier) -> Self {
        ModuleId { address, name }
    }

    pub fn name(&self) -> &IdentStr {
        &self.name
    }

    pub fn address(&self) -> &AccountAddress {
        &self.address
    }

    pub fn access_vector(&self) -> Vec<u8> {
        let mut key = vec![CODE_TAG];
        key.append(&mut bcs::to_bytes(self).unwrap());
        key
    }

    pub fn to_canonical_string(&self, with_prefix: bool) -> String {
        self.to_canonical_display(with_prefix).to_string()
    }

    /// Proxy type for overriding `ModuleId`'s display implementation, to use a
    /// canonical form (full-width addresses), with an optional "0x" prefix
    /// (controlled by the `with_prefix` flag).
    pub fn to_canonical_display(&self, with_prefix: bool) -> impl Display + '_ {
        struct IdDisplay<'a> {
            id: &'a ModuleId,
            with_prefix: bool,
        }

        impl<'a> Display for IdDisplay<'a> {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}::{}",
                    self.id.address.to_canonical_display(self.with_prefix),
                    self.id.name,
                )
            }
        }

        IdDisplay {
            id: self,
            with_prefix,
        }
    }
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_canonical_display(/* with_prefix */ false))
    }
}

impl FromStr for ModuleId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParsedModuleId::parse(s)?.into_module_id(&|_| None)
    }
}

impl ModuleId {
    pub fn short_str_lossless(&self) -> String {
        format!("0x{}::{}", self.address.short_str_lossless(), self.name)
    }
}

impl Display for StructTag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "0x{}::{}::{}",
            self.address.short_str_lossless(),
            self.module,
            self.name
        )?;
        if let Some(first_ty) = self.type_params.first() {
            write!(f, "<")?;
            write!(f, "{}", first_ty)?;
            for ty in self.type_params.iter().skip(1) {
                write!(f, ", {}", ty)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl Display for TypeTag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TypeTag::Struct(s) => write!(f, "{}", s),
            TypeTag::Vector(ty) => write!(f, "vector<{}>", ty),
            TypeTag::U8 => write!(f, "u8"),
            TypeTag::U16 => write!(f, "u16"),
            TypeTag::U32 => write!(f, "u32"),
            TypeTag::U64 => write!(f, "u64"),
            TypeTag::U128 => write!(f, "u128"),
            TypeTag::U256 => write!(f, "u256"),
            TypeTag::Address => write!(f, "address"),
            TypeTag::Signer => write!(f, "signer"),
            TypeTag::Bool => write!(f, "bool"),
        }
    }
}

impl From<StructTag> for TypeTag {
    fn from(t: StructTag) -> TypeTag {
        TypeTag::Struct(Box::new(t))
    }
}

