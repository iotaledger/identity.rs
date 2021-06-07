// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

/// A DIDComm Timing Decorator
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Field_Definitions.md)
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
  wait_until_time: Option<Timestamp>,
  #[serde(skip_serializing_if = "Option::is_none")]
  delay_milli: Option<u32>,
}

impl Timing {
  /// Creates a new `Timing` decorator.
  pub fn new() -> Self {
    Self {
      out_time: None,
      in_time: None,
      stale_time: None,
      expires_time: None,
      wait_until_time: None,
      delay_milli: None,
    }
  }

  /// Returns a reference to the `out_time` field.
  pub fn out_time(&self) -> Option<&Timestamp> {
    self.out_time.as_ref()
  }

  /// Returns a mutable reference to the `out_time` field.
  pub fn out_time_mut(&mut self) -> Option<&mut Timestamp> {
    self.out_time.as_mut()
  }

  /// Sets the value of the `out_time` field.
  pub fn set_out_time<T: Into<Option<Timestamp>>>(&mut self, value: T) {
    self.out_time = value.into();
  }

  /// Returns a reference to the `in_time` field.
  pub fn in_time(&self) -> Option<&Timestamp> {
    self.in_time.as_ref()
  }

  /// Returns a mutable reference to the `in_time` field.
  pub fn in_time_mut(&mut self) -> Option<&mut Timestamp> {
    self.in_time.as_mut()
  }

  /// Sets the value of the `in_time` field.
  pub fn set_in_time<T: Into<Option<Timestamp>>>(&mut self, value: T) {
    self.in_time = value.into();
  }

  /// Returns a reference to the `stale_time` field.
  pub fn stale_time(&self) -> Option<&Timestamp> {
    self.stale_time.as_ref()
  }

  /// Returns a mutable reference to the `stale_time` field.
  pub fn stale_time_mut(&mut self) -> Option<&mut Timestamp> {
    self.stale_time.as_mut()
  }

  /// Sets the value of the `stale_time` field.
  pub fn set_stale_time<T: Into<Option<Timestamp>>>(&mut self, value: T) {
    self.stale_time = value.into();
  }

  /// Returns a reference to the `expires_time` field.
  pub fn expires_time(&self) -> Option<&Timestamp> {
    self.expires_time.as_ref()
  }

  /// Returns a mutable reference to the `expires_time` field.
  pub fn expires_time_mut(&mut self) -> Option<&mut Timestamp> {
    self.expires_time.as_mut()
  }

  /// Sets the value of the `expires_time` field.
  pub fn set_expires_time<T: Into<Option<Timestamp>>>(&mut self, value: T) {
    self.expires_time = value.into();
  }

  /// Returns a reference to the `wait_until_time` field.
  pub fn wait_until_time(&self) -> Option<&Timestamp> {
    self.wait_until_time.as_ref()
  }

  /// Returns a mutable reference to the `wait_until_time` field.
  pub fn wait_until_time_mut(&mut self) -> Option<&mut Timestamp> {
    self.wait_until_time.as_mut()
  }

  /// Sets the value of the `wait_until_time` field.
  pub fn set_wait_until_time<T: Into<Option<Timestamp>>>(&mut self, value: T) {
    self.wait_until_time = value.into();
  }

  /// Returns the value of the `delay_milli` field.
  pub fn delay_milli(&self) -> Option<u32> {
    self.delay_milli
  }

  /// Sets the value of the `delay_milli` field.
  pub fn set_delay_milli<T: Into<Option<u32>>>(&mut self, value: T) {
    self.delay_milli = value.into();
  }
}
