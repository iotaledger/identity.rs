// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod identity;
mod identity_client;
mod identity_client_builder;
mod multicontroller;

pub use identity::*;
pub use identity_client::*;
pub use identity_client_builder::*;
pub use multicontroller::*;

// dummy types, have to be replaced with actual types later on
pub type DummySigner = str;
pub type Hashable<T> = Vec<T>;
pub type Identity = ();
