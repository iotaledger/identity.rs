use identity_core::common::Object;

use crate::error::Result;

pub trait ProofDocument {
  fn to_object(&self) -> Result<Object>;
}
