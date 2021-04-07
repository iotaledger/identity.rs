// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example goes into more detail regarding the useage of DID Documents.
//!
//! cargo run --example manipulate_did_document

use identity::crypto::KeyPair;
use identity::iota::Document;
use identity::iota::Result;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a DID Document out of an ed25519 keypair.
  let keypair: KeyPair = KeyPair::new_ed25519()?;
  let mut document: identity::iota::Document = Document::from_keypair(&keypair)?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;
  println!("DID Document (signed) > {:#}", document);
  println!();

  // We can access individual fields of the DID Document as defined below using the appropriate getters:
  let _did: &identity::did::DID = document.id(); // The Document ID.
  let _controller: Option<&identity::did::DID> = document.controller(); // The Document controller.
  let _aka: &[identity::core::Url] = document.also_known_as(); // AKA: Subject of this identifier is also identified by one or more other identifiers.
                                                               // ... etc. Each getter also has a _mut variant returning a mutable reference instead of an immutable one, e.g.
                                                               // .id_mut() See also https://identity.docs.iota.org/docs/identity/did/struct.Document.html

  // We can iterate over a DID Document's verification methods using document.methods(), which returns an iterator:
  for m in document.methods() {
    // m is of type &identity::did::Method
    println!("Method > {:#}", m);
  }
  println!();

  // We can also add and remove methods from a DID Document using insert_method() and remove_method() respectively.
  let method: &identity::did::Method = document.methods().next().unwrap();
  let _method_did: &identity::did::DID = method.id();
  // document.remove_method(identity::iota::DID::new(public: &[u8]));

  // We can search for a specific method using .resolve(), for instance if we want to have the first #authentication
  // method, we can use:
  let auth_meth = document.resolve("#authentication").unwrap();
  println!("Authentication Method > {:#}", auth_meth);
  println!();

  // We can sign arbitrary structs using the DID Document signer if they implement the trait
  // `identity::crypto::SetSignature`
  //
  // e.g. document.signer(keypair.secret()).method("#authentication").sign(&mut test);

  Ok(())
}
