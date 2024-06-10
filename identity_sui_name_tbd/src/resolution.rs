// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::pin::Pin;

use futures::stream::FuturesUnordered;
use futures::Future;
use futures::TryStreamExt;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::SuiClient;

use crate::migration::get_alias;
use crate::migration::get_identity as get_did;
use crate::migration::lookup;
use crate::migration::Identity;
use crate::Error;

pub async fn get_identity(client: &SuiClient, object_id: ObjectID) -> Result<Identity, Error> {
  // spawn all checks
  let mut all_futures =
    FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>> + Send>>>::new();
  all_futures.push(Box::pin(resolve_new(client, object_id)));
  all_futures.push(Box::pin(resolve_migrated(client, object_id)));
  all_futures.push(Box::pin(resolve_unmigrated(client, object_id)));

  // use first non-None value as result
  let mut state_metadata_outcome: Option<Identity> = None;
  while let Some(result) = all_futures.try_next().await? {
    if result.is_some() {
      state_metadata_outcome = result;
      all_futures.clear();
      break;
    }
  }

  // check if we found state metadata
  let state_metadata = if let Some(value) = state_metadata_outcome {
    value
  } else {
    return Err(Error::DIDResolutionErrorKinesis(format!(
      "could not find DID document for {object_id}"
    )));
  };

  // unpack and return document
  Ok(state_metadata)
}

async fn resolve_new(client: &SuiClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let on_chain_identity = get_did(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;

  Ok(on_chain_identity.map(Identity::FullFledged))
}

async fn resolve_migrated(client: &SuiClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let migrated_identity = lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionErrorKinesis(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;

  Ok(migrated_identity.map(Identity::FullFledged))
}

async fn resolve_unmigrated(client: &SuiClient, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let unmigrated_alias = get_alias(client, object_id)
    .await
    .map_err(|err| Error::DIDResolutionErrorKinesis(format!("could  no query for object id {object_id}; {err}")))?;
  unmigrated_alias
    .map(|v| {
      v.state_metadata.map(Identity::RawDid).ok_or_else(|| {
        Error::DIDResolutionErrorKinesis(format!(
          "unmigrated alias for object id {object_id} does not contain DID document"
        ))
      })
    })
    .transpose()
}
