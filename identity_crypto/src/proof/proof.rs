use crate::{
  error::{Error, Result},
  identity_core::{Object, Timestamp},
  key::{KeyGenerator, KeyPair, PublicKey, SecretKey},
  proof::{LinkedDataProof, ProofDocument, ProofOptions},
  signature::SignatureSuite,
  utils::{decode_b64, encode_b64},
};

pub struct Proof(Box<dyn SignatureSuite>);

impl Proof {
  pub const DEFAULT_PURPOSE: &'static str = "assertionMethod";

  pub fn new(suite: impl SignatureSuite + 'static) -> Self {
    Self(Box::new(suite))
  }

  pub fn signature_type(&self) -> &'static str {
    self.0.signature_type()
  }

  pub fn key_type(&self) -> &'static str {
    self.0.key_type()
  }

  pub fn keypair(&self, generator: KeyGenerator) -> Result<KeyPair> {
    self.0.keypair(generator)
  }

  pub fn sign(
    &self,
    document: &dyn ProofDocument,
    secret: &SecretKey,
    mut options: ProofOptions,
  ) -> Result<LinkedDataProof> {
    // Apply default options
    if options.created.is_none() {
      options.created = Some(Timestamp::now());
    }

    if options.proof_purpose.is_none() {
      options.proof_purpose = Some(Self::DEFAULT_PURPOSE.into());
    }

    // Normalize the document
    let data: Vec<u8> = self.to_hash(document, &options)?;

    // Sign the document and encode in a web-safe format
    let signature: Vec<u8> = self.0.sign(&data, secret)?;
    let signature: String = encode_b64(&signature);

    // Construct and return the proof docment
    Ok(LinkedDataProof {
      type_: self.signature_type().into(),
      proof_purpose: options.proof_purpose.expect("infallible"),
      proof_value: signature.into(),
      verification_method: options.verification_method,
      created: options.created.expect("infallible"),
      domain: options.domain,
      nonce: options.nonce,
      properties: Default::default(),
    })
  }

  pub fn verify(&self, document: &dyn ProofDocument, proof: &LinkedDataProof, public: &PublicKey) -> Result<bool> {
    // Convert proof to options
    let options: ProofOptions = proof.to_options();

    // Normalize the document
    let data: Vec<u8> = self.to_hash(document, &options)?;

    // Decode the proof signature
    let signature: Vec<u8> = decode_b64(&proof.proof_value)?;

    // Actually do the verification
    self.0.verify(&data, &signature, public)
  }

  // Create a verification hash for the given document/options
  //
  // Ref: https://w3c-ccg.github.io/ld-proofs/#create-verify-hash-algorithm
  fn to_hash(&self, document: &dyn ProofDocument, options: &dyn ProofDocument) -> Result<Vec<u8>> {
    let mut options_digest: Vec<u8> = options
      .to_object()
      .and_then(|options| self.canonicalize_options(options))
      .and_then(|options| self.0.digest(&options))?;

    let mut document_digest: Vec<u8> = document
      .to_object()
      .and_then(|document| self.canonicalize_document(document))
      .and_then(|document| self.0.digest(&document))?;

    options_digest.append(&mut document_digest);

    Ok(options_digest)
  }

  fn canonicalize_options(&self, object: Object) -> Result<Vec<u8>> {
    self.canonicalize(object, &["created"], &["type", "id", "proofValue", "jws"])
  }

  fn canonicalize_document(&self, object: Object) -> Result<Vec<u8>> {
    self.canonicalize(object, &[], &["proof"])
  }

  fn canonicalize(&self, mut object: Object, required: &[&str], excluded: &[&str]) -> Result<Vec<u8>> {
    for key in required {
      if !object.contains_key(*key) {
        return Err(Error::InvalidProofDocument(format!("Missing `{}`", key)));
      }
    }

    for key in excluded {
      object.remove(*key);
    }

    self.0.canonicalize(object)
  }
}
