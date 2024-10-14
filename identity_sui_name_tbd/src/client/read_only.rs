use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::str::FromStr;

use anyhow::Context as _;
use futures::stream::FuturesUnordered;
use futures::TryStreamExt as _;
use identity_iota_core::block::output::AliasId;
use identity_iota_core::IotaDID;
use identity_iota_core::IotaDocument;
use identity_iota_core::NetworkName;
use identity_iota_core::StateMetadataDocument;
use crate::sui::iota_sdk_adapter::IotaClientTraitCore;
use crate::iota_sdk_abstraction::rpc_types::EventFilter;
use crate::iota_sdk_abstraction::rpc_types::IotaData as _;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectData;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectDataFilter;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectDataOptions;
use crate::iota_sdk_abstraction::rpc_types::IotaObjectResponseQuery;
use crate::iota_sdk_abstraction::rpc_types::OwnedObjectRef;
use crate::iota_sdk_abstraction::types::base_types::IotaAddress;
use crate::iota_sdk_abstraction::types::base_types::ObjectID;
use crate::iota_sdk_abstraction::types::base_types::ObjectRef;
use crate::iota_sdk_abstraction::move_types::language_storage::StructTag;
use serde::Deserialize;

use crate::migration::get_alias;
use crate::migration::get_identity;
use crate::migration::lookup;
use crate::migration::Identity;
use crate::Error;

const UNKNOWN_NETWORK_HRP: &str = "unknwn";

/// An [`IotaClient`] enriched with identity-related
/// functionalities.
#[derive(Clone)]
pub struct IdentityClientReadOnly<C> {
  iota_client: C,
  identity_iota_pkg_id: ObjectID,
  migration_registry_id: ObjectID,
  network: NetworkName,
}

impl<C: IotaClientTraitCore> Deref for IdentityClientReadOnly<C> {
  type Target = C;
  fn deref(&self) -> &Self::Target {
    &self.iota_client
  }
}

impl<C> IdentityClientReadOnly<C> {
  /// Returns `identity_iota`'s package ID.
  /// The ID of the packages depends on the network
  /// the client is connected to.
  pub const fn package_id(&self) -> ObjectID {
    self.identity_iota_pkg_id
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
}

impl<C: IotaClientTraitCore + Sync> IdentityClientReadOnly<C> {
  /// Attempts to create a new [`IdentityClientReadOnly`] from
  /// the given [`IotaClient`].
  pub async fn new(iota_client: C, identity_iota_pkg_id: ObjectID) -> Result<Self, Error> {
    let IdentityPkgMetadata {
      migration_registry_id, ..
    } = identity_pkg_metadata(&iota_client, identity_iota_pkg_id).await?;
    let network = get_client_network(&iota_client).await?;
    Ok(Self {
      iota_client,
      identity_iota_pkg_id,
      migration_registry_id,
      network,
    })
  }

  /// Same as [`Self::new`], but if the network isn't recognized among IOTA's official networks,
  /// the provided `network_name` will be used.
  pub async fn new_with_network_name(
    iota_client: C,
    identity_iota_pkg_id: ObjectID,
    network_name: NetworkName,
  ) -> Result<Self, Error> {
    let mut identity_client = Self::new(iota_client, identity_iota_pkg_id).await?;
    if identity_client.network.as_ref() == UNKNOWN_NETWORK_HRP {
      identity_client.network = network_name;
    }

    Ok(identity_client)
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
      .and_then(|obj| serde_json::from_value(obj.fields.to_json_value()).context("failed to deserialize move object"))
      .map_err(|e| Error::ObjectLookup(e.to_string()))
  }

  #[allow(dead_code)]
  pub(crate) async fn get_object_ref_by_id(&self, obj: ObjectID) -> Result<Option<OwnedObjectRef>, Error> {
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
    let identity = get_identity(self, get_object_id_from_did(did)?)
      .await?
      .ok_or_else(|| Error::DIDResolutionError(format!("call succeeded but could not resolve {did} to object")))?;
    let state_metadata = identity.multicontroller().controlled_value();

    // return empty document if disabled
    if state_metadata.is_empty() {
      let mut empty_document = IotaDocument::new_with_id(did.clone());
      empty_document.metadata.created = None;
      empty_document.metadata.updated = None;
      empty_document.metadata.deactivated = Some(true);

      return Ok(empty_document);
    }

    // unpack, replace placeholders and return document
    StateMetadataDocument::unpack(state_metadata)
      .and_then(|doc| doc.into_iota_document(did))
      .map_err(|err| {
        Error::DidDocParsingFailed(format!(
          "could not transform DID document to IotaDocument for DID {did}; {err}"
        ))
      })
  }

  /// Resolves an [`Identity`] from its ID `object_id`.
  pub async fn get_identity(&self, object_id: ObjectID) -> Result<Identity, Error> {
    // spawn all checks
    let mut all_futures =
      FuturesUnordered::<Pin<Box<dyn Future<Output = Result<Option<Identity>, Error>> + Send>>>::new();
    all_futures.push(Box::pin(resolve_new(self, object_id)));
    all_futures.push(Box::pin(resolve_migrated(self, object_id)));
    all_futures.push(Box::pin(resolve_unmigrated(self, object_id)));

    // use first non-None value as result
    let mut identity_outcome: Option<Identity> = None;
    while let Some(result) = all_futures.try_next().await? {
      if result.is_some() {
        identity_outcome = result;
        all_futures.clear();
        break;
      }
    }

    identity_outcome.ok_or_else(|| Error::DIDResolutionError(format!("could not find DID document for {object_id}")))
  }
}

#[derive(Debug)]
struct IdentityPkgMetadata {
  _package_id: ObjectID,
  migration_registry_id: ObjectID,
}

#[derive(Deserialize)]
struct MigrationRegistryCreatedEvent {
  #[allow(dead_code)]
  id: ObjectID,
}

async fn get_client_network<C: IotaClientTraitCore>(client: &C) -> Result<NetworkName, Error> {
  let network_id = client
    .read_api()
    .get_chain_identifier()
    .await
    .map_err(|e| Error::RpcError(e.to_string()))?;

  // TODO: add entries when identity_iota package is published to well-known networks.
  #[allow(clippy::match_single_binding)]
  let network_hrp = match &network_id {
    // "89c3eeec" => NetworkName::try_from("iota").unwrap(),
    // "fe12a865" => NetworkName::try_from("atoi").unwrap(),
    _ => NetworkName::try_from(UNKNOWN_NETWORK_HRP).unwrap(), // Unrecognized network
  };

  Ok(network_hrp)
}

// TODO: remove argument `package_id` and use `EventFilter::MoveEventField` to find the beacon event and thus the
// package id.
// TODO: authenticate the beacon event with though sender's ID.
async fn identity_pkg_metadata<C: IotaClientTraitCore>(iota_client: &C, package_id: ObjectID) -> Result<IdentityPkgMetadata, Error> {
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
      "no \"identity_iota\" package found on the provided network".to_string(),
    ));
  };

  let registry_id = serde_json::from_value::<MigrationRegistryCreatedEvent>(event.parsed_json)
    .map(|e| e.id)
    .map_err(|e| {
      Error::MigrationRegistryNotFound(crate::migration::Error::NotFound(format!(
        "Malformed \"MigrationRegistryEvent\": {}",
        e
      )))
    })?;

  Ok(IdentityPkgMetadata {
    migration_registry_id: registry_id,
    _package_id: package_id,
  })
}

async fn resolve_new<C: IotaClientTraitCore>(client: &IdentityClientReadOnly<C>, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = get_identity(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "could not get identity document for object id {object_id}; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_migrated<C: IotaClientTraitCore>(client: &IdentityClientReadOnly<C>, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let onchain_identity = lookup(client, object_id).await.map_err(|err| {
    Error::DIDResolutionError(format!(
      "failed to look up object_id {object_id} in migration registry; {err}"
    ))
  })?;
  Ok(onchain_identity.map(Identity::FullFledged))
}

async fn resolve_unmigrated<C: IotaClientTraitCore>(client: &IdentityClientReadOnly<C>, object_id: ObjectID) -> Result<Option<Identity>, Error> {
  let unmigrated_alias = get_alias(client.deref(), object_id)
    .await
    .map_err(|err| Error::DIDResolutionError(format!("could  no query for object id {object_id}; {err}")))?;
  Ok(unmigrated_alias.map(Identity::Legacy))
}

pub fn get_object_id_from_did(did: &IotaDID) -> Result<ObjectID, Error> {
  ObjectID::from_str(&AliasId::from(did).to_string())
    .map_err(|err| Error::DIDResolutionError(format!("could not parse object id from did {did}; {err}")))
}
