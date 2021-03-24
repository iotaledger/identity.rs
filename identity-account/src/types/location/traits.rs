// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub trait ToKey {
  const TAG: char;

  fn id(&self) -> String;

  fn to_key(&self) -> String {
    format!("{}:{}", Self::TAG, self.id())
  }
}
