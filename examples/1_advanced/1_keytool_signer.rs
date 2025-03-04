// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use identity_iota::iota::rebased::client::IdentityClient;
use identity_iota::iota::rebased::client::IdentityClientReadOnly;
use identity_iota::iota::rebased::transaction::Transaction;
use identity_iota::iota::IotaDocument;
use identity_iota::iota_interaction::KeytoolSignerBuilder;
use iota_sdk::IotaClientBuilder;
use iota_sdk::IOTA_LOCAL_NETWORK_URL;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let api_endpoint = std::env::var("API_ENDPOINT").unwrap_or_else(|_| IOTA_LOCAL_NETWORK_URL.to_string());
  let iota_client = IotaClientBuilder::default().build(api_endpoint).await?;

  let identity_client = {
    let package_id = std::env::var("IOTA_IDENTITY_PKG_ID")
      .context("IOTA_IDENTITY_PKG_ID must be set in order to run the examples")?
      .parse()?;
    let read_only_client = IdentityClientReadOnly::new_with_pkg_id(iota_client, package_id).await?;

    // Use `iota` binary in PATH and active address to sign transactions.
    let keytool_signer = KeytoolSignerBuilder::default().build().await?;

    IdentityClient::new(read_only_client, keytool_signer).await?
  };

  let identity = identity_client
    .create_identity(IotaDocument::new(identity_client.network()))
    .finish()
    .execute(&identity_client)
    .await?
    .output;

  println!(
    "Created a new Identity {} with {} as controller.",
    identity.did_document().id(),
    identity_client.sender_address()
  );

  Ok(())
}
