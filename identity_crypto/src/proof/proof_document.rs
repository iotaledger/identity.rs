use crate::{error::Result, identity_core::Object};

pub trait ProofDocument {
  fn to_object(&self) -> Result<Object>;
}
