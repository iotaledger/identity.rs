// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use identity_iota::iota::IotaDID;
use identity_iota::iota::IotaDocument;
use identity_iota::prelude::Resolver;
use sui_sdk::SuiClientBuilder;

/// Demonstrates how to resolve an existing DID in an Alias Output.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let client = SuiClientBuilder::default()
    .build("http://127.0.0.1:9000")
    .await
    .map_err(|err| anyhow!(format!("failed to connect to network; {}", err)))?;

  // We can also create a `Resolver` that has additional convenience methods,
  // for example to resolve presentation issuers or to verify presentations.
  let mut resolver = Resolver::<IotaDocument>::new();

  // We need to register a handler that can resolve IOTA DIDs.
  // This convenience method only requires us to provide a client.
  resolver.attach_kinesis_iota_handler(client.clone());

  let did = IotaDID::parse("did:iota:0x49f0cf61970ccf7006da235fcef43802b85c4bf09dd587db7f5cfe629b3bd1da")?;

  let result = resolver.resolve(&did).await;

  dbg!(&result);

  assert!(result.is_ok());

  Ok(())
}
