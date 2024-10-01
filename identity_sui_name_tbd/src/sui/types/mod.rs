#![allow(unused_imports)]
mod number;

use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::id::UID;
pub use number::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bag {
  pub id: UID,
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  pub size: u64,
}

impl Default for Bag {
  fn default() -> Self {
    Self {
      id: UID::new(ObjectID::ZERO),
      size: 0,
    }
  }
}
