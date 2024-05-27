// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use identity_sui_name_tbd::migration::get_alias;
use identity_sui_name_tbd::migration::lookup;
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
    let alias_id: AliasId = AliasId::from(did);
    let alias_id_string = alias_id.to_string();

    // try to resolve unmigrated alias (stardust `Alias` object)
    let unmigrated_alias = get_alias(self, &alias_id_string).await.map_err(|err| {
      Error::DIDResolutionErrorKinesis(format!("could  no query for alias output {alias_id_string}; {err}"))
    })?;
    let state_metadata = if let Some(unmigrated_alias_value) = unmigrated_alias {
      // if we found an unmigrated alias, fetch state metadata / serialized document from it
      unmigrated_alias_value
        .state_metadata
        .ok_or_else(|| Error::DIDResolutionErrorKinesis("alias state metadata must not be empty".to_string()))?
    } else {
      // otherwise check registry for a migrated alias
      let object_id = ObjectID::from_str(&alias_id_string)
        .map_err(|_| Error::DIDSyntaxError(identity_did::Error::InvalidMethodId))?;
      let document = lookup(self, object_id)
        .await
        .map_err(|err| {
          Error::DIDResolutionErrorKinesis(format!("failed to look up alias id in migration registry; {err}"))
        })?
        .ok_or(Error::DIDResolutionErrorKinesis(format!(
          "could not find alias id {alias_id_string} in migration registry"
        )))?;
      // if we found a mapping, resolve to identity package `Document` object
      // and get state metadata / serialized document from it
      document.doc
    };

    // unpack and return document
    return StateMetadataDocument::unpack(&state_metadata).and_then(|doc| doc.into_iota_document(did));
  }
}
