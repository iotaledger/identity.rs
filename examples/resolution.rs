// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! cargo run --example resolution

use identity::crypto::KeyPair;
use identity::did::resolution::dereference;
use identity::did::resolution::resolve;
use identity::did::resolution::Dereference;
use identity::did::resolution::Resolution;
use identity::iota::Client;
use identity::iota::IotaDID;
use identity::iota::IotaDocument;
use identity::iota::Result;

#[smol_potat::main]
async fn main() -> Result<()> {
  let client: Client = Client::new()?;

  // Generate a new DID Document and public/private key pair.
  //
  // The generated document will have an authentication key associated with
  // the keypair.
  let (mut document, keypair): (IotaDocument, KeyPair) = IotaDocument::builder()
    .authentication_tag("key-1")
    .did_network(client.network().as_str())
    .build()?;

  // Sign the DID Document with the default authentication key.
  document.sign(keypair.secret())?;

  println!("DID Document (signed) > {:#}", document);
  println!();

  // Use the client created above to publish the DID Document to the Tangle.
  document.publish_with_client(&client).await?;

  let did: &IotaDID = document.id();

  // Resolve the DID and retrieve the published DID Document from the Tangle.
  let resolution: Resolution = resolve(did.as_str(), Default::default(), &client).await?;

  println!("DID Document Resolution > {:#?}", resolution);
  println!();

  // Dereference the DID and retrieve the authentication method generated above.
  let did: IotaDID = did.join("#key-1")?;
  let dereference: Dereference = dereference(did.as_str(), Default::default(), &client).await?;

  println!("DID Document Dereference > {:#?}", dereference);
  println!();

  Ok(())
}
