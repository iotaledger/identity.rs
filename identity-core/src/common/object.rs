// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

#[doc(inline)]
pub use serde_json::Value;

/// An alias for an ordered map of key-[value][`Value`] pairs.
pub type Object = BTreeMap<String, Value>;
