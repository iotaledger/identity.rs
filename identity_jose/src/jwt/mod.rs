// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! JSON Web Tokens ([JWT](https://tools.ietf.org/html/rfc7519))

mod claims;
mod header;

pub use self::claims::*;
pub use self::header::*;
