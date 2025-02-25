// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod identity;
mod multicontroller;

pub(crate) use identity::*;
pub(crate) use multicontroller::*;

// dummy types, have to be replaced with actual types later on
pub(crate) type Hashable<T> = Vec<T>;
