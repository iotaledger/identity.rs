// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;

use crate::rebased::iota;
use crate::IotaDID;
use crate::IotaDocument;
use crate::NetworkName;
use anyhow::anyhow;
use anyhow::Context as _;
use futures::stream::FuturesUnordered;

use identity_core::common::Url;
use identity_did::DID;
use futures::StreamExt as _;
use iota_sdk::rpc_types::EventFilter;
use iota_sdk::rpc_types::IotaData as _;
use iota_sdk::rpc_types::IotaObjectData;
use iota_sdk::rpc_types::IotaObjectDataFilter;
use iota_sdk::rpc_types::IotaObjectDataOptions;
use iota_sdk::rpc_types::IotaObjectResponseQuery;
use iota_sdk::rpc_types::OwnedObjectRef;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::base_types::ObjectRef;
use iota_sdk::IotaClient;
use move_core_types::language_storage::StructTag;
use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::rebased::migration::get_alias;
use crate::rebased::migration::get_identity;
use crate::rebased::migration::lookup;
use crate::rebased::migration::Identity;
use crate::rebased::Error;

/// An [`IotaClient`] enriched with identity-related
/// functionalities.
#[derive(Clone)]
pub struct IdentityClientReadOnly {
  iota_client: IotaClient,
  iota_identity_pkg_id: ObjectID,
  migration_registry_id: ObjectID,
  network: NetworkName,
}

impl Deref for IdentityClientReadOnly {
  type Target = IotaClient;
  fn deref(&self) -> &Self::Target {
    &self.iota_client
  }
}

impl IdentityClientReadOnly {
  /// Returns `iota_identity`'s package ID.
  /// The ID of the packages depends on the network
  /// the client is connected to.
  pub const fn package_id(&self) -> ObjectID {
    self.iota_identity_pkg_id
  }

  /// Returns the name of the network the client is
  /// currently connected to.
  pub const fn network(&self) -> &NetworkName {
    &self.network
  }

  /// Returns the migration registry's ID.
  pub const fn migration_registry_id(&self) -> ObjectID {
    self.migration_registry_id
  }

  /// Attempts to create a new [`IdentityClientReadOnly`] from a given [`IotaClient`].

  /// # Failures
  /// This function fails if the provided `iota_client` is connected to an unrecognized
  /// network.
  ///
  /// # Notes
  /// When trying to connect to a local or unofficial network prefer using
  /// [`IdentityClientReadOnly::new_with_pkg_id`].
  pub async fn new(iota_client: IotaClient) -> Result<Self, Error> {
    let network = network_id(&iota_client).await?;
    let metadata = iota::well_known_networks::network_metadata(&network).ok_or_else(|| {
      Error::InvalidConfig(format!(
        "unrecognized network \"{network}\". Use `new_with_pkg_id` instead."
      ))
    })?;

    let pkg_id = metadata.latest_pkg_id();

    Ok(IdentityClientReadOnly {
      iota_client,
      iota_identity_pkg_id: pkg_id,
      migration_registry_id: metadata.migration_registry(),
      network,
    })
  }

  /// Attempts to create a new [`IdentityClientReadOnly`] from
  /// the given [`IotaClient`].
  pub async fn new_with_pkg_id(iota_client: IotaClient, iota_identity_pkg_id: ObjectID) -> Result<Self, Error> {
    let IdentityPkgMetadata {
      migration_registry_id, ..
    } = identity_pkg_metadata(&iota_client, iota_identity_pkg_id).await?;
    let network = network_id(&iota_client).await?;
    Ok(Self {
      iota_client,
      iota_identity_pkg_id,
      migration_registry_id,
      network,
    })
  }

  /// Resolves a _Move_ Object of ID `id` and parses it to a value of type `T`.
  pub async fn get_object_by_id<T>(&self, id: ObjectID) -> Result<T, Error>
  where
    T: DeserializeOwned,
  {
    self
      .read_api()
      .get_object_with_options(id, IotaObjectDataOptions::new().with_content())
      .await
      .context("lookup request failed")
      .and_then(|res| res.data.context("missing data in response"))
      .and_then(|data| data.content.context("missing object content in data"))
      .and_then(|content| content.try_into_move().context("not a move object"))
      .and_then(|obj| {
        serde_json::from_value(obj.fields.to_json_value())
          .map_err(|err| anyhow!("failed to deserialize move object; {err}"))
      })
      .map_err(|e| Error::ObjectLookup(e.to_string()))
  }

  /// Returns an object's [`OwnedObjectRef`], if any.
  pub async fn get_object_ref_by_id(&self, obj: ObjectID) -> Result<Option<OwnedObjectRef>, Error> {
    self
      .read_api()
      .get_object_with_options(obj, IotaObjectDataOptions::default().with_owner())
      .await
      .map(|response| {
        response.data.map(|obj_data| OwnedObjectRef {
          owner: obj_data.owner.expect("requested data"),
          reference: obj_data.object_ref().into(),
        })
      })
      .map_err(Error::from)
  }

  /// Queries the object owned by this sender address and returns the first one
  /// that matches `tag` and for which `predicate` returns `true`.
  pub async fn find_owned_ref_for_address<P>(
    &self,
    address: IotaAddress,
    tag: StructTag,
    predicate: P,
  ) -> Result<Option<ObjectRef>, Error>
  where
    P: Fn(&IotaObjectData) -> bool,
  {
    let filter = IotaObjectResponseQuery::new_with_filter(IotaObjectDataFilter::StructType(tag));

    let mut cursor = None;
    loop {
      let mut page = self
        .read_api()
        .get_owned_objects(address, Some(filter.clone()), cursor, None)
        .await?;
      let obj_ref = std::mem::take(&mut page.data)
        .into_iter()
        .filter_map(|res| res.data)
        .find(|obj| predicate(obj))
        .map(|obj_data| obj_data.object_ref());
      cursor = page.next_cursor;

      if obj_ref.is_some() {
        return Ok(obj_ref);
      }
      if !page.has_next_page {
        break;
      }
    }

    Ok(None)
  }

  /// Queries an [`IotaDocument`] DID Document through its `did`.
  pub async fn resolve_did(&self, did: &IotaDID) -> Result<IotaDocument, Error> {
    self
      .get_identity(get_object_id_from_did(did)?)
      .await?
      .did_document(self.network())
  }

  /// Resolves an [`Identity`] from its ID `object_id`.
  pub async fn get_identity(&self, object_id: ObjectID) -> Result<Identity, Error> {
    // spawn all checks
    let all_futures =
      FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>> + Send>>>::new();
    all_futures.push(Box::pin(resolve_new(self, object_id)));
    all_futures.push(Box::pin(resolve_migrated(self, object_id)));
    all_futures.push(Box::pin(resolve_unmigrated(self, object_id)));

    all_futures
      .filter_map(|res| Box::pin(async move { res.ok().flatten() }))
      .next()
      .await
      .ok_or_else(|| Error::DIDResolutionError(format!("could not find DID document for {object_id}")))
  }
}

async fn network_id(iota_client: &IotaClient) -> Result<NetworkName, Error> {
  let network_id = iota_client
    .read_api()
    .get_chain_identifier()
    .await
    .map_err(|e| Error::RpcError(e.to_string()))?;
  Ok(network_id.try_into().expect("chain ID is a valid network name"))
}

#[derive(Debug)]
struct IdentityPkgMetadata {
  migration_registry_id: ObjectID,
}

#[derive(Deserialize)]
struct MigrationRegistryCreatedEvent {
  #[allow(dead_code)]
  id: ObjectID,
}

// TODO: remove argument `package_id` and use `EventFilter::MoveEventField` to find the beacon event and thus the
// package id.
// TODO: authenticate the beacon event with though sender's ID.
async fn identity_pkg_metadata(iota_client: &IotaClient, package_id: ObjectID) -> Result<IdentityPkgMetadata, Error> {
  // const EVENT_BEACON_PATH: &str = "/beacon";
  // const EVENT_BEACON_VALUE: &[u8] = b"identity.rs_pkg";

  // let event_filter = EventFilter::MoveEventField {
  //   path: EVENT_BEACON_PATH.to_string(),
  //   value: EVENT_BEACON_VALUE.to_json_value().expect("valid json representation"),
  // };
  let event_filter = EventFilter::MoveEventType(
    StructTag::from_str(&format!("{package_id}::migration_registry::MigrationRegistryCreated")).expect("valid utf8"),
  );
  let mut returned_events = iota_client
    .event_api()
    .query_events(event_filter, None, Some(1), false)
    .await
    .map_err(|e| Error::RpcError(e.to_string()))?
    .data;
  let event = if !returned_events.is_empty() {
    returned_events.swap_remove(0)
  } else {
    return Err(Error::InvalidConfig(
      "no \"iota_identity\" package found on the provided network".to_string(),
    ));
  };

  let registry_id = serde_json::from_value::<MigrationRegistryCreatedEvent>(event.parsed_json)
    .map(|e| e.id)
    .map_err(|e| {
      Error::MigrationRegistryNotFound(crate::rebased::migration::Error::NotFound(format!(
        "Malformed \"MigrationRegistryEvent\": {}",
        e
      )))
    })?;

  Ok(IdentityPkgMetadata {
    migration_registry_id: registry_id,
  })
}

async fn resolve_new(client: &IdentityClientReadOnly, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = get_identity(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_migrated(client: &IdentityClientReadOnly, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;
  let Some(mut onchain_identity) = onchain_identity else {
    return Ok(None);
  };
  let object_id_str = object_id.to_string();
  let queried_did = IotaDID::from_object_id(&object_id_str, &client.network);
  let identity_did = onchain_identity.did_document().id().clone();
  let doc = onchain_identity.did_document_mut();

  // When querying a migrated identity we obtain a DID document with DID `identity_did` and the `alsoKnownAs`
  // property containing `queried_did`. Since we are resolving `queried_did`, lets replace in the document these values.
  // `queried_id` becomes the DID Document ID.
  *doc.core_document_mut().id_mut_unchecked() = queried_did.clone().into();
  // The DID Document `alsoKnownAs` property is cleaned of its `queried_did` entry,
  // which gets replaced by `identity_did`.
  doc
    .also_known_as_mut()
    .replace::<Url>(&queried_did.into_url().into(), identity_did.into_url().into());

  Ok(Some(Identity::FullFledged(onchain_identity)))
}

async fn resolve_unmigrated(client: &IdentityClientReadOnly, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let unmigrated_alias = get_alias(client, object_id)
    .await
    .map_err(|err| Error::DIDResolutionError(format!("could  no query for object id {object_id}; {err}")))?;
  Ok(unmigrated_alias.map(Identity::Legacy))
}

/// Extracts the object ID from the given `IotaDID`.
///
/// # Arguments
///
/// * `did` - A reference to the `IotaDID` to be converted.
pub fn get_object_id_from_did(did: &IotaDID) -> Result<ObjectID, Error> {
  ObjectID::from_str(did.tag_str())
    .map_err(|err| Error::DIDResolutionError(format!("could not parse object id from did {did}; {err}")))
}
