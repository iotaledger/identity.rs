use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use crate::{
  error::{Error, Result},
  proof::Proof,
};

type ProofBuilder = fn() -> Proof;
type ProofTypes = HashMap<&'static str, ProofBuilder>;

lazy_static! {
  static ref PROOF_TYPES: Arc<RwLock<ProofTypes>> = { Arc::new(RwLock::new(HashMap::new())) };
}

#[derive(Clone, Copy, Debug)]
pub struct ProofManager;

impl ProofManager {
  pub fn add(name: &'static str, builder: ProofBuilder) -> Result<()> {
    PROOF_TYPES
      .write()
      .map_err(|_| Error::RwLockPoisonedWrite)?
      .insert(name, builder);

    Ok(())
  }

  pub fn all() -> Result<Vec<&'static str>> {
    PROOF_TYPES
      .read()
      .map_err(|_| Error::RwLockPoisonedRead)
      .map(|suites| suites.keys().copied().collect())
  }

  pub fn get(name: &(impl AsRef<str> + ?Sized)) -> Result<Proof> {
    PROOF_TYPES
      .read()
      .map_err(|_| Error::RwLockPoisonedRead)?
      .get(name.as_ref())
      .ok_or_else(|| Error::MissingProofType(name.as_ref().into()))
      .map(|builder| builder())
  }
}
