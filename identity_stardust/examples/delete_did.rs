// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stardust::StardustClientExt;
use identity_stardust::StardustDocument;
use iota_client::block::address::Address;
use iota_client::secret::SecretManager;
use iota_client::Client;

mod create_did;

/// Demonstrate how to destroy an existing DID in an Alias Output, reclaiming the stored deposit.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new DID in an Alias Output for us to modify.
  let (client, address, secret_manager, document): (Client, Address, SecretManager, StardustDocument) =
    create_did::run().await?;

  client.delete_did(&secret_manager, address, document.id()).await?;

  Ok(())
}
