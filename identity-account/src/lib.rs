// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

pub mod account;
pub mod crypto;
pub mod error;
pub mod events;
pub mod identity;
pub mod storage;
pub mod stronghold;
pub mod types;
pub mod utils;

pub use self::error::Error;
pub use self::error::Result;
