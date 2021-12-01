// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Re-exports public errors from this crate
mod fatal_error;
pub use crate::common::TimeStampParsingError;
pub use crate::convert::JsonDecodingError;
pub use crate::convert::JsonEncodingError;
pub use crate::crypto::merkle_key::MerkleDigestKeyTagError;
pub use crate::crypto::merkle_key::MerkleKeyTagExtractionError;
pub use crate::crypto::merkle_key::MerkleSignatureKeyTagError;
pub use crate::crypto::KeyCollectionError;
pub use crate::crypto::KeyCollectionSizeError;
pub use crate::crypto::KeyPairGenerationError;
pub use crate::crypto::MissingSignatureError;
pub use crate::crypto::ProofValueError;
pub use crate::crypto::SigningError;
pub use crate::crypto::VerificationError;
pub use crate::crypto::VerificationProcessingError;
pub use crate::utils::Base58DecodingError;
pub use crate::utils::Base64DecodingError;
pub use crate::utils::MultiBaseDecodingError;
pub use fatal_error::FatalError;

/// Re-export of `ParseError` from the `url` crate.
// The `url` crate is stable with millions of downloads so we consider it fine to include this in our public API.
pub type UrlParsingError = url::ParseError;
