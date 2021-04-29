// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

/// A DIDComm Timing
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Interactions%20and%20Messages.md)
///
#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq)]
pub struct Timing {
  #[serde(skip_serializing_if = "Option::is_none")]
  out_time: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  in_time: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  stale_time: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  expires_time: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  delay_milli: Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  wait_until_time: Option<Timestamp>,
}

impl Timing {
  pub fn new() -> Self {
    Self {
      out_time: None,
      in_time: None,
      stale_time: None,
      expires_time: None,
      delay_milli: None,
      wait_until_time: None,
    }
  }

  /// Get a mutable reference to the timing's out time.
  pub fn out_time_mut(&mut self) -> &mut Option<Timestamp> {
    &mut self.out_time
  }

  /// Get a reference to the timing's out time.
  pub fn out_time(&self) -> &Option<Timestamp> {
    &self.out_time
  }

  /// Set the timing's out time.
  pub fn set_out_time(&mut self, out_time: Option<Timestamp>) {
    self.out_time = out_time;
  }

  /// Get a mutable reference to the timing's in time.
  pub fn in_time_mut(&mut self) -> &mut Option<Timestamp> {
    &mut self.in_time
  }

  /// Get a reference to the timing's in time.
  pub fn in_time(&self) -> &Option<Timestamp> {
    &self.in_time
  }

  /// Set the timing's in time.
  pub fn set_in_time(&mut self, in_time: Option<Timestamp>) {
    self.in_time = in_time;
  }

  /// Get a mutable reference to the timing's stale time.
  pub fn stale_time_mut(&mut self) -> &mut Option<Timestamp> {
    &mut self.stale_time
  }

  /// Get a reference to the timing's stale time.
  pub fn stale_time(&self) -> &Option<Timestamp> {
    &self.stale_time
  }

  /// Set the timing's stale time.
  pub fn set_stale_time(&mut self, stale_time: Option<Timestamp>) {
    self.stale_time = stale_time;
  }

  /// Get a mutable reference to the timing's expires time.
  pub fn expires_time_mut(&mut self) -> &mut Option<Timestamp> {
    &mut self.expires_time
  }

  /// Get a reference to the timing's expires time.
  pub fn expires_time(&self) -> &Option<Timestamp> {
    &self.expires_time
  }

  /// Set the timing's expires time.
  pub fn set_expires_time(&mut self, expires_time: Option<Timestamp>) {
    self.expires_time = expires_time;
  }

  /// Get a mutable reference to the timing's delay milli.
  pub fn delay_milli_mut(&mut self) -> &mut Option<i32> {
    &mut self.delay_milli
  }

  /// Get a reference to the timing's delay milli.
  pub fn delay_milli(&self) -> &Option<i32> {
    &self.delay_milli
  }

  /// Set the timing's delay milli.
  pub fn set_delay_milli(&mut self, delay_milli: Option<i32>) {
    self.delay_milli = delay_milli;
  }

  /// Get a mutable reference to the timing's wait until time.
  pub fn wait_until_time_mut(&mut self) -> &mut Option<Timestamp> {
    &mut self.wait_until_time
  }

  /// Get a reference to the timing's wait until time.
  pub fn wait_until_time(&self) -> &Option<Timestamp> {
    &self.wait_until_time
  }

  /// Set the timing's wait until time.
  pub fn set_wait_until_time(&mut self, wait_until_time: Option<Timestamp>) {
    self.wait_until_time = wait_until_time;
  }
}
