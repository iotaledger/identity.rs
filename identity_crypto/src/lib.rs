#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

pub mod error;
pub mod key;
pub mod proof;
pub mod signature;
pub mod utils;

pub mod identity_core {
  #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
  pub struct Object;

  #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
  pub struct Timestamp;
}
