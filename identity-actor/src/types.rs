// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::firewall::{PermissionValue, RequestPermissions, VariantPermission};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, RequestPermissions)]
pub struct NamedMessage {
  pub name: String,
  pub data: Vec<u8>,
}

impl NamedMessage {
  pub fn new<S: Into<String>>(name: S, data: Vec<u8>) -> Self {
    Self {
      name: name.into(),
      data,
    }
  }
}
