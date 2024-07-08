// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_trait::async_trait;
use identity_sui_name_tbd::client::IdentityClient;
use identity_sui_name_tbd::client::KinesisKeySignature;
use iota_sdk::types::block::output::AliasId;
use secret_storage::signer::Signer;
use sui_sdk::types::base_types::ObjectID;

use crate::Error;
use crate::IotaDID;
use crate::IotaDocument;
use crate::Result;
use crate::StateMetadataDocument;

/// An extension trait that provides helper functions forasync_trait resolution of DID documents in unmigrated Alias
/// Outputs and migrated identity document.
#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
pub trait KinesisIotaIdentityClientExt {
  /// Resolve a [`IotaDocument`].
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument>;

  /// Publish the given document and return document as it would be received when resolving.
  ///
  /// This means that placeholder
  /// DIDs stored when publishing to network but are replaced in the document that is returned from here and from
  /// resolving.
  async fn publish_did_document<S>(&self, document: IotaDocument, gas_budget: u64, signer: &S) -> Result<IotaDocument>
  where
    S: Signer<KinesisKeySignature>;
}

#[cfg_attr(not(feature = "send-sync-storage"), async_trait(?Send))]
#[cfg_attr(feature = "send-sync-storage", async_trait)]
impl KinesisIotaIdentityClientExt for IdentityClient {
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument> {
    // get alias id from did (starting with 0x)
    let object_id = ObjectID::from_str(&AliasId::from(did).to_string())
      .map_err(|_| Error::DIDResolutionErrorKinesis(format!("could not parse object id from did {did}")))?;

    let state_metadata = self.get_raw_did_document(object_id).await.unwrap();

    // unpack, replace placeholders and return document
    return StateMetadataDocument::unpack(&state_metadata).and_then(|doc| doc.into_iota_document(did));
  }

  async fn publish_did_document<S>(&self, document: IotaDocument, gas_budget: u64, signer: &S) -> Result<IotaDocument>
  where
    S: Signer<KinesisKeySignature>,
  {
    let packed = document.clone().pack().unwrap();

    let document_id: ObjectID = self
      .publish_raw_did_document(&packed, gas_budget, signer)
      .await
      .unwrap();

    // replace placeholders in document
    let did: IotaDID = IotaDID::new(&document_id.into_bytes(), &self.network_name().try_into().unwrap());
    let metadata_document: StateMetadataDocument = document.into();
    let document_without_placeholders = metadata_document.into_iota_document(&did).unwrap();

    Ok(document_without_placeholders)
  }
}
