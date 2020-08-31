use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use crate::{error::Result, proof::Proof};

type ProofBuilder = fn() -> Proof;
type ProofTypes = HashMap<&'static str, ProofBuilder>;

lazy_static! {
  static ref PROOF_TYPES: Arc<RwLock<ProofTypes>> = { Arc::new(RwLock::new(HashMap::new())) };
}

#[derive(Clone, Copy, Debug)]
pub struct ProofManager;

impl ProofManager {
  pub fn add(name: &'static str, builder: ProofBuilder) -> Result<()> {
    todo!()
  }

  pub fn all() -> Result<Vec<&'static str>> {
    todo!()
  }

  pub fn get(name: &str) -> Result<Proof> {
    todo!()
  }
}
