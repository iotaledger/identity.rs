// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use embedded_revocation_list::EmbeddedRevocationList;
pub use embedded_revocation_list::EMBEDDED_REVOCATION_METHOD_NAME;
pub use error::RevocationMethodError;

mod embedded_revocation_list;
mod error;
