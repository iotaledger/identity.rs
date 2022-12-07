// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use identity_core::utils::Base;

/// Parameters for KMS generation of key material 
/// compliant with the `MultiKey` format. 
pub struct MultikeySchema {
    // TODO: Is there a better (internal representation than String)?
    // Maybe unsigned varint? 
    // Maybe enum with hardcoded values from https://github.com/multiformats/multicodec/blob/master/table.csv 
    // Maybe Type wrapping a Cow<'static, str> with associated constants corresponding to values in the aforementioned table? 
    multi_codec_code: Cow<'static, str>, 
    multibase_identifier: Base
}

impl MultikeySchema {
    pub fn codec_code_str(&self) -> &str {
        &self.multi_codec_code
    }

    pub fn multibase_identifier(&self) -> Base {
        self.multibase_identifier
    }

    /// Creates a [`MultikeySchema`] representing parameters for generating an `Ed25519` public key with base58-encoding using the Bitcoin base-encoding alphabet.
    pub fn ed25519_public_key() -> Self {
        Self { multi_codec_code: Cow::Borrowed("0xed"), multibase_identifier: Base::Base58Btc }
    }

}
