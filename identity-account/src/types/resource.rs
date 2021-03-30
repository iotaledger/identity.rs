// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resource {
  Chain,
  Event,
}

impl Resource {
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Chain => "Chain",
      Self::Event => "Event",
    }
  }
}
