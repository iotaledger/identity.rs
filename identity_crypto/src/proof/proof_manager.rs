use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use crate::{
  error::{Error, Result},
  proof::Proof,
  signature::{EcdsaSecp256k1, Ed25519},
};

type ProofBuilder = fn() -> Proof;
type ProofTypes = HashMap<&'static str, ProofBuilder>;

lazy_static! {
  static ref PROOF_TYPES: Arc<RwLock<ProofTypes>> = {
    fn create_ed25519() -> Proof {
      Proof::new(Ed25519)
    }

    fn create_ecdsa_secp256k1() -> Proof {
      Proof::new(EcdsaSecp256k1)
    }

    let mut types: ProofTypes = HashMap::new();

    // TODO: Register these under a common key? - possibly using an enum with a
    // flexible/user-friendly FromStr implementation

    types.insert("Ed25519", create_ed25519);
    types.insert("Ed25519Signature2018", create_ed25519);
    types.insert("Ed25519VerificationKey2018", create_ed25519);

    types.insert("EcdsaSecp256k1", create_ecdsa_secp256k1);
    types.insert("EcdsaSecp256k1Signature2019", create_ecdsa_secp256k1);
    types.insert("EcdsaSecp256k1VerificationKey2019", create_ecdsa_secp256k1);

    Arc::new(RwLock::new(types))
  };
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
