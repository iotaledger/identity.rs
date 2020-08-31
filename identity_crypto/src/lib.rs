#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub mod canonicalize;
pub mod error;
pub mod key;
pub mod proof;
pub mod signature;
pub mod utils;

pub mod identity_core {
  use std::collections::HashMap;
  use serde_json::Value;

  pub type Object = HashMap<String, Value>;

  #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
  pub struct Timestamp;

  impl Timestamp {
    pub fn now() -> Self {
      Self
    }
  }
}
