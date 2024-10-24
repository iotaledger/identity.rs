// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::ToJson;
use identity_iota::iota::{IotaDocument, NetworkName};
use identity_stronghold::StrongholdStorage;
use identity_sui_name_tbd::utils::request_funds;
use iota_sdk::types::block::address::ToBech32Ext;
use tonic::Request;

use crate::helpers::make_stronghold;
use crate::helpers::Entity;
use crate::helpers::TestServer;
use crate::helpers::FAUCET_ENDPOINT;
use crate::helpers::{get_address, get_address_with_funds};
use _document::document_service_client::DocumentServiceClient;
use _document::CreateDidRequest;

mod _document {
  tonic::include_proto!("document");
}

#[tokio::test]
async fn did_document_creation() -> anyhow::Result<()> {
  let stronghold = StrongholdStorage::new(make_stronghold());
  let server = TestServer::new_with_stronghold(stronghold.clone()).await;

  let user = Entity::new_with_stronghold(stronghold);
  let (user_address, key_id, _) = get_address(user.storage()).await?;

  request_funds(&user_address).await?;

  let mut grpc_client = DocumentServiceClient::connect(server.endpoint()).await?;
  grpc_client
    .create(Request::new(CreateDidRequest {
      key_id: key_id.as_str().to_string(),
    }))
    .await?;

  Ok(())
}

#[test]
fn lol() {
  pub const TEST_DOC: &[u8] = &[
    68, 73, 68, 1, 0, 131, 1, 123, 34, 100, 111, 99, 34, 58, 123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58,
    48, 34, 44, 34, 118, 101, 114, 105, 102, 105, 99, 97, 116, 105, 111, 110, 77, 101, 116, 104, 111, 100, 34, 58, 91,
    123, 34, 105, 100, 34, 58, 34, 100, 105, 100, 58, 48, 58, 48, 35, 79, 115, 55, 95, 66, 100, 74, 120, 113, 86, 119,
    101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86, 113, 100, 108, 74, 73, 99, 48,
    45, 100, 50, 49, 73, 34, 44, 34, 99, 111, 110, 116, 114, 111, 108, 108, 101, 114, 34, 58, 34, 100, 105, 100, 58,
    48, 58, 48, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 74, 115, 111, 110, 87, 101, 98, 75, 101, 121, 34, 44, 34,
    112, 117, 98, 108, 105, 99, 75, 101, 121, 74, 119, 107, 34, 58, 123, 34, 107, 116, 121, 34, 58, 34, 79, 75, 80, 34,
    44, 34, 97, 108, 103, 34, 58, 34, 69, 100, 68, 83, 65, 34, 44, 34, 107, 105, 100, 34, 58, 34, 79, 115, 55, 95, 66,
    100, 74, 120, 113, 86, 119, 101, 76, 107, 56, 73, 87, 45, 76, 71, 83, 111, 52, 95, 65, 115, 52, 106, 70, 70, 86,
    113, 100, 108, 74, 73, 99, 48, 45, 100, 50, 49, 73, 34, 44, 34, 99, 114, 118, 34, 58, 34, 69, 100, 50, 53, 53, 49,
    57, 34, 44, 34, 120, 34, 58, 34, 75, 119, 99, 54, 89, 105, 121, 121, 65, 71, 79, 103, 95, 80, 116, 118, 50, 95, 49,
    67, 80, 71, 52, 98, 86, 87, 54, 102, 89, 76, 80, 83, 108, 115, 57, 112, 122, 122, 99, 78, 67, 67, 77, 34, 125, 125,
    93, 125, 44, 34, 109, 101, 116, 97, 34, 58, 123, 34, 99, 114, 101, 97, 116, 101, 100, 34, 58, 34, 50, 48, 50, 52,
    45, 48, 53, 45, 50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 44, 34, 117, 112, 100, 97, 116, 101, 100, 34,
    58, 34, 50, 48, 50, 52, 45, 48, 53, 45, 50, 50, 84, 49, 50, 58, 49, 52, 58, 51, 50, 90, 34, 125, 125,
  ];

  // convert to string
  let did_value = IotaDocument::new(&NetworkName::try_from("iota").unwrap()).to_json().unwrap();

  // println!("did_value : {}", did_value.to_string().unwrap());

  println!("DID VALUE {did_value}");
}
