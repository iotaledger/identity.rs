// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Timestamp;

#[derive(Debug, Deserialize, Serialize, Default)]
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
