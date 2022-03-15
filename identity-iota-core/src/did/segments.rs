// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::did::IotaDID;

macro_rules! get {
  (@network $this:expr) => {
    &$this.0[..get!(@head $this)]
  };
  (@tag $this:expr) => {
    &$this.0[get!(@tail $this) + 1..]
  };
  (@head $this:expr) => {
    // unwrap is fine - we only operate on valid DIDs
    $this.0.find(':').unwrap()
  };
  (@tail $this:expr) => {
    // unwrap is fine - we only operate on valid DIDs
    $this.0.rfind(':').unwrap()
  };
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segments<'id>(pub(crate) &'id str);

impl<'id> Segments<'id> {
  pub fn is_default_network(&self) -> bool {
    match self.count() {
      1 => true,
      2 => get!(@network self) == IotaDID::DEFAULT_NETWORK,
      _ => unreachable!("Segments::is_default_network called for invalid IOTA DID"),
    }
  }

  pub fn network(&self) -> &'id str {
    match self.count() {
      1 => IotaDID::DEFAULT_NETWORK,
      2 => get!(@network self),
      _ => unreachable!("Segments::network called for invalid IOTA DID"),
    }
  }

  pub fn tag(&self) -> &'id str {
    match self.count() {
      1 => self.0,
      2 => get!(@tag self),
      _ => unreachable!("Segments::tag called for invalid IOTA DID"),
    }
  }

  pub fn count(&self) -> usize {
    self.0.split(':').count()
  }
}
