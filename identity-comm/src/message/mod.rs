// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::module_inception)]

mod authentication;
mod credential_issuance;
mod credential_options;
mod credential_revocation;
mod credential_schema;
mod discovery;
mod message;
mod presentation_verification;
mod report;
mod resolution;
mod timing;
mod trustping;

pub use self::authentication::*;
pub use self::credential_issuance::*;
pub use self::credential_options::*;
pub use self::credential_revocation::*;
pub use self::credential_schema::*;
pub use self::discovery::*;
pub use self::message::*;
pub use self::presentation_verification::*;
pub use self::report::*;
pub use self::resolution::*;
pub use self::timing::*;
pub use self::trustping::*;
