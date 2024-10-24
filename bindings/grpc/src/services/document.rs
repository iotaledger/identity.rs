// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use _document::document_service_server::DocumentService;
use _document::document_service_server::DocumentServiceServer;
use _document::CreateDidRequest;
use _document::CreateDidResponse;
use identity_iota::core::ToJson;
use identity_iota::iota::IotaDocument;
use identity_iota::iota::IotaIdentityClientExt;
use identity_iota::iota::{IotaClientExt, IotaDID};
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkStorageDocumentError;
use identity_iota::storage::Storage;
use identity_iota::verification::jws::JwsAlgorithm;
use identity_iota::verification::{MethodScope, VerificationMethod};
use identity_storage::{KeyId, StorageSigner};
use identity_stronghold::StrongholdStorage;
use identity_stronghold::ED25519_KEY_TYPE;
use identity_sui_name_tbd::client::{IdentityClient, IdentityClientReadOnly};
use identity_sui_name_tbd::transaction::Transaction;
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
  client: IdentityClientReadOnly,
}

impl DocumentSvc {
  pub fn new(client: &IdentityClientReadOnly, stronghold: &StrongholdStorage) -> Self {
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
    let CreateDidRequest { key_id } = req.into_inner();

    let key_id = KeyId::new(key_id);
    let pub_key = self.storage.key_id_storage().get_public_key(&key_id).await.unwrap();

    let network_name = self.client.network();

    let storage = StorageSigner::new(&self.storage, key_id, pub_key);

    let identity_client = IdentityClient::new(self.client.clone(), storage).await.unwrap();

    let iota_doc = IotaDocument::new(network_name).to_json().unwrap();
    let mut doc = vec![];
    doc.extend_from_slice(b"DID"); // Add the DID Marker
    doc.extend_from_slice(&iota_doc.as_bytes());

    println!("Creating identity with doc: {:?}", doc);

    let mut created_identity = identity_client
      .create_identity(&doc)
      .finish()
      .execute(&identity_client)
      .await
      .unwrap();

    let did = IotaDID::parse(format!("did:iota:{}", created_identity.id())).unwrap();

    let mut document = IotaDocument::new_with_id(did.clone());
    let fragment = document
      .generate_method(
        &self.storage,
        ED25519_KEY_TYPE.clone(),
        JwsAlgorithm::EdDSA,
        Some(identity_client.signer().key_id().as_str()),
        MethodScope::VerificationMethod,
      )
      .await
      .map_err(Error::StorageError)?;

    created_identity
      .update_did_document(document.clone())
      .finish()
      .execute(&identity_client)
      .await
      .unwrap();

    Ok(Response::new(CreateDidResponse {
      document_json: document.to_json().unwrap(),
      fragment,
      did: did.to_string(),
    }))
  }
}

pub fn service(client: &IdentityClientReadOnly, stronghold: &StrongholdStorage) -> DocumentServiceServer<DocumentSvc> {
  DocumentServiceServer::new(DocumentSvc::new(client, stronghold))
}
