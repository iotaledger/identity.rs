// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![deny(clippy::all)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate napi_derive;

pub mod account;
pub mod did;
pub mod error;
