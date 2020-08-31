#[macro_use]
extern crate identity_core;

use identity_crypto::{key::*, proof::*};
use identity_vc::prelude::*;
use serde_json::to_string_pretty;

fn main() -> anyhow::Result<()> {
  let credential: Credential = CredentialBuilder::new()
    .context("did:iota:university:5678")
    .id("did:iota:cred:1234")
    .issuer("did:iota:ident:university")
    .type_("UniversityDegreeCredential")
    .try_subject(object!(id: "did:iota:ident:alice", alumniOf: "ExampleUniversity"))?
    .try_issuance_date("2020-01-01T00:00:00Z")?
    .property("foo", 1234)
    .property("bar", vec![5, 6, 7, 8])
    .non_transferable("YES")
    .build()?;

  println!("[+] Available Proofs > {:?}", ProofManager::all()?);

  let suite: Proof = ProofManager::get("EcdsaSecp256k1")?; // Ed25519
  let keypair: KeyPair = suite.keypair(KeyGenerator::None)?;

  println!("[+] Public Key > {}", keypair.public());
  println!("[+] Secret Key > {}", keypair.secret());

  let proof: LinkedDataProof = suite.sign(&credential, keypair.secret(), Default::default())?;

  println!("[+] Linked Data Proof > {:#?}", proof);

  let verifiable: VerifiableCredential = VerifiableCredential::new(credential, proof);

  println!("[+] Verifiable Credential > {:#?}", verifiable);

  let verified: bool = verifiable.verify(|value: &str| -> Result<PublicKey> { Ok(keypair.public().clone()) })?;

  println!("[+] Verified > {}", verified);

  println!("[+] VC JSON > {}", to_string_pretty(&verifiable)?);

  Ok(())
}
