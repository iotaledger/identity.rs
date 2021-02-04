// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A basic example that generates a DID Document, publishes it to the Tangle,
//! and retrieves information through DID Document resolution/dereferencing.
//!
//! cargo run --example resolution

mod common;

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

  // Create a new DID Document, signed and published.
  let doc: IotaDocument = common::document(&client).await?.0;
  let did: &IotaDID = doc.id();

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
