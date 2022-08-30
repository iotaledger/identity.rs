// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod error;
mod resolution;

pub use self::error::Error;
pub use self::error::ErrorCause;
pub use self::error::ResolutionAction;
pub use self::error::Result;
pub use resolution::*;
