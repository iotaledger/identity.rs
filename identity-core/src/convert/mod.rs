// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Traits for JSON conversions between types.

mod json;
mod serde_into;

pub use self::json::FromJson;
pub use self::json::ToJson;
pub use self::serde_into::SerdeInto;
