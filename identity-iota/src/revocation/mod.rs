// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub use error::RevocationError;
pub use simple_revocation_list::SimpleRevocationList2022;
pub(crate) use simple_revocation_list::SIMPLE_REVOCATION_METHOD_NAME;
pub use traits::RevocationMethod;

mod error;
mod simple_revocation_list;
mod traits;
