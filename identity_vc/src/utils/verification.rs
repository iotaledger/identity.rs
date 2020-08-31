use identity_crypto::{
  key::PublicKey,
  proof::{LinkedDataProof, Proof, ProofDocument, ProofManager},
};

use crate::{common::OneOrMany, error::Result};

pub fn verify_document(
  document: &dyn ProofDocument,
  proofs: &OneOrMany<LinkedDataProof>,
  resolve: impl Fn(&str) -> Result<PublicKey>,
) -> Result<bool> {
  for proof in proofs.iter() {
    let suite: Proof = ProofManager::get(&proof.type_)?;
    let public: PublicKey = resolve(&proof.verification_method)?;
    let verified: bool = suite.verify(document, &proof, &public)?;

    if !verified {
      return Ok(false);
    }
  }

  Ok(true)
}
