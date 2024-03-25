// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use _document::document_service_server::DocumentService;
use _document::document_service_server::DocumentServiceServer;
use _document::CreateDidRequest;
use _document::CreateDidResponse;
use identity_iota::core::ToJson;
use identity_iota::iota::IotaClientExt;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkStorageDocumentError;
use identity_iota::storage::Storage;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::MethodScope;
use identity_stronghold::StrongholdStorage;
use identity_stronghold::ED25519_KEY_TYPE;
use iota_sdk::client::Client;
use iota_sdk::types::block::address::Address;
use std::error::Error as _;
use tonic::Code;
use tonic::Request;
use tonic::Response;
use tonic::Status;

mod _document {
  tonic::include_proto!("document");
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("The provided address is not a valid bech32 encoded address")]
  InvalidAddress,
  #[error(transparent)]
  IotaClientError(identity_iota::iota::Error),
  #[error(transparent)]
  StorageError(JwkStorageDocumentError),
}

impl From<Error> for Status {
  fn from(value: Error) -> Self {
    let code = match &value {
      Error::InvalidAddress => Code::InvalidArgument,
      _ => Code::Internal,
    };
    Status::new(code, value.to_string())
  }
}

pub struct DocumentSvc {
  storage: Storage<StrongholdStorage, StrongholdStorage>,
  client: Client,
}

impl DocumentSvc {
  pub fn new(client: &Client, stronghold: &StrongholdStorage) -> Self {
    Self {
      storage: Storage::new(stronghold.clone(), stronghold.clone()),
      client: client.clone(),
    }
  }
}

#[tonic::async_trait]
impl DocumentService for DocumentSvc {
  #[tracing::instrument(
    name = "create_did_document",
    skip_all,
    fields(request = ?req.get_ref())
    ret,
    err,
  )]
  async fn create(&self, req: Request<CreateDidRequest>) -> Result<Response<CreateDidResponse>, Status> {
    let CreateDidRequest { bech32_address } = req.into_inner();
    let address = Address::try_from_bech32(&bech32_address).map_err(|_| Error::InvalidAddress)?;
    let network_name = self.client.network_name().await.map_err(Error::IotaClientError)?;

    let mut document = IotaDocument::new(&network_name);
    let fragment = document
      .generate_method(
        &self.storage,
        ED25519_KEY_TYPE.clone(),
        JwsAlgorithm::EdDSA,
        None,
        MethodScope::VerificationMethod,
      )
      .await
      .map_err(Error::StorageError)?;

    let alias_output = self
      .client
      .new_did_output(address, document, None)
      .await
      .map_err(Error::IotaClientError)?;

    let document = self
      .client
      .publish_did_output(self.storage.key_storage().as_secret_manager(), alias_output)
      .await
      .map_err(Error::IotaClientError)
      .inspect_err(|e| tracing::error!("{:?}", e.source()))?;
    let did = document.id();

    Ok(Response::new(CreateDidResponse {
      document_json: document.to_json().unwrap(),
      fragment,
      did: did.to_string(),
    }))
  }
}

pub fn service(client: &Client, stronghold: &StrongholdStorage) -> DocumentServiceServer<DocumentSvc> {
  DocumentServiceServer::new(DocumentSvc::new(client, stronghold))
}
