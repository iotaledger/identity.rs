// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_sui_name_tbd::resolution::get_did_document;
use iota_sdk::types::block::output::AliasId;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::SuiClient;

use crate::Error;
use crate::IotaDID;
use crate::IotaDocument;
use crate::Result;
use crate::StateMetadataDocument;

/// An extension trait that provides helper functions for resolution of DID documents in unmigrated Alias Outputs and
/// migrated identity document.
#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
pub trait KinesisIotaIdentityClientExt {
  /// Resolve a [`IotaDocument`].
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument>;
}

#[cfg_attr(feature = "send-sync-client-ext", async_trait::async_trait)]
#[cfg_attr(not(feature = "send-sync-client-ext"), async_trait::async_trait(?Send))]
impl KinesisIotaIdentityClientExt for SuiClient {
  async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument> {
    // get alias id from did (starting with 0x)
    let object_id = ObjectID::from_str(&AliasId::from(did).to_string())
      .map_err(|_| Error::DIDResolutionErrorKinesis(format!("could not parse object id from did {did}")))?;

    let state_metadata = get_did_document(self, object_id).await.unwrap();

    // unpack and return document
    return StateMetadataDocument::unpack(&state_metadata).and_then(|doc| doc.into_iota_document(did));
  }
}
