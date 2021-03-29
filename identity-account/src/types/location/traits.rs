// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::ResourceType;

pub trait ToKey {
  fn type_(&self) -> ResourceType;

  fn id(&self) -> String;

  fn to_key(&self) -> String {
    format!("{}:{}", self.type_().name(), self.id())
  }
}
