// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_grpc::server::GRpcServer;
use identity_stronghold::StrongholdStorage;
use iota_sdk::client::stronghold::StrongholdAdapter;
use iota_sdk::client::Client;

use identity_sui_name_tbd::client::{IdentityClient, IdentityClientReadOnly};
use iota_sdk_move::types::base_types::ObjectID;

#[tokio::main]
#[tracing::instrument(err)]
async fn main() -> anyhow::Result<()> {
  tracing::subscriber::set_global_default(tracing_subscriber::fmt().compact().finish())
    .expect("Failed to setup global tracing subscriber.");

  let api_endpoint = std::env::var("API_ENDPOINT")?;

  let identity_iota_pkg_id = std::env::var("IDENTITY_IOTA_PKG_ID")?;

  let identity_pkg_id = ObjectID::from_str(&identity_iota_pkg_id)?;

  // let client: Client = Client::builder()
  //   .with_primary_node(&api_endpoint, None)?
  //   .finish()
  //   .await?;
  let iota_client = iota_sdk_move::IotaClientBuilder::default().build(api_endpoint).await?;

  let read_only_client = IdentityClientReadOnly::new(iota_client, identity_pkg_id).await?;

  let stronghold = init_stronghold()?;

  let addr = "0.0.0.0:50051".parse()?;
  tracing::info!("gRPC server listening on {}", addr);
  GRpcServer::new(read_only_client, stronghold).serve(addr).await?;

  Ok(())
}

#[tracing::instrument]
fn init_stronghold() -> anyhow::Result<StrongholdStorage> {
  let stronghold_password = std::env::var("STRONGHOLD_PWD")?;
  let snapshot_path = std::env::var("SNAPSHOT_PATH")?;

  // Check for snapshot file at specified path
  let metadata = std::fs::metadata(&snapshot_path)?;
  if !metadata.is_file() {
    return Err(anyhow::anyhow!("No snapshot at provided path \"{}\"", &snapshot_path));
  }

  Ok(
    StrongholdAdapter::builder()
      .password(stronghold_password)
      .build(snapshot_path)
      .map(StrongholdStorage::new)?,
  )
}
