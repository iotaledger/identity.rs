// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::ident_str;

use super::super::super::move_core_types::language_storage::StructTag;
use super::super::super::move_core_types::identifier::IdentStr;
use super::super::STARDUST_PACKAGE_ID;

pub const IRC27_MODULE_NAME: &IdentStr = ident_str!("irc27");
pub const NFT_MODULE_NAME: &IdentStr = ident_str!("nft");
pub const NFT_OUTPUT_MODULE_NAME: &IdentStr = ident_str!("nft_output");
pub const NFT_OUTPUT_STRUCT_NAME: &IdentStr = ident_str!("NftOutput");
pub const NFT_STRUCT_NAME: &IdentStr = ident_str!("Nft");
pub const IRC27_STRUCT_NAME: &IdentStr = ident_str!("Irc27Metadata");
pub const NFT_DYNAMIC_OBJECT_FIELD_KEY: &[u8] = b"nft";
pub const NFT_DYNAMIC_OBJECT_FIELD_KEY_TYPE: &str = "vector<u8>";

pub struct Nft {}

impl Nft {
    /// Returns the struct tag that represents the fully qualified path of an
    /// [`Nft`] in its move package.
    pub fn tag() -> StructTag {
        StructTag {
            address: STARDUST_PACKAGE_ID.into(),
            module: NFT_MODULE_NAME.to_owned(),
            name: NFT_STRUCT_NAME.to_owned(),
            type_params: Vec::new(),
        }
    }
}