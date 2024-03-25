// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_stronghold::StrongholdStorage;
use iota_sdk::types::block::address::ToBech32Ext;
use tonic::Request;

use crate::helpers::get_address_with_funds;
use crate::helpers::make_stronghold;
use crate::helpers::Entity;
use crate::helpers::TestServer;
use crate::helpers::FAUCET_ENDPOINT;
use _document::document_service_client::DocumentServiceClient;
use _document::CreateDidRequest;

mod _document {
  tonic::include_proto!("document");
}

#[tokio::test]
async fn did_document_creation() -> anyhow::Result<()> {
  let stronghold = StrongholdStorage::new(make_stronghold());
  let server = TestServer::new_with_stronghold(stronghold.clone()).await;
  let api_client = server.client();
  let hrp = api_client.get_bech32_hrp().await?;

  let user = Entity::new_with_stronghold(stronghold);
  let user_address = get_address_with_funds(
    api_client,
    user.storage().key_storage().as_secret_manager(),
    FAUCET_ENDPOINT,
  )
  .await?;

  let mut grpc_client = DocumentServiceClient::connect(server.endpoint()).await?;
  grpc_client
    .create(Request::new(CreateDidRequest {
      bech32_address: user_address.to_bech32(hrp).to_string(),
    }))
    .await?;

  Ok(())
}
