// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod full_client;
mod read_only;

pub use full_client::*;
use iota_sdk::types::base_types::IotaAddress;
use iota_sdk::types::crypto::PublicKey;
pub use read_only::*;

pub use identity_iota_interaction::IotaKeySignature;

use async_trait::async_trait;
use identity_iota_interaction::types::base_types::{ObjectID, ObjectRef};

use serde::de::DeserializeOwned;

use crate::iota_interaction_adapter::IotaClientAdapter; // Need access to the adapter

use crate::rebased::Error;
use crate::NetworkName;

/// A trait defining the core functionalities required for interacting
/// with IOTA identities, suitable for both read-only and full clients.
#[async_trait(?Send)]
pub trait CoreClient<S>: Sized + Clone {
  /// Returns the underlying IOTA client adapter.
  fn client_adapter(&self) -> &IotaClientAdapter;

  /// Returns the sender address of the client.
  fn sender_address(&self) -> IotaAddress;

  /// Returns the public key of the client.
  fn public_key(&self) -> PublicKey;

  /// Returns a reference to this [`CoreClient`]'s [`Signer`].
  fn signer(&self) -> &S;

  /// Returns `iota_identity`'s package ID.
  fn package_id(&self) -> ObjectID;

  /// Returns the name of the network the client is connected to.
  fn network(&self) -> &NetworkName;

  /// Resolves a _Move_ Object of ID [id](cci:1://file:///Users/yasirdev/Documents/Work/IOTA/identity-rs/identity_iota_core/src/rebased/client/read_only.rs:67:2-72:3) and parses it to a value of type `T`.
  async fn get_object_by_id<T>(&self, id: ObjectID) -> Result<T, Error>
  where
    T: DeserializeOwned;

  /// Returns an object's [`ObjectRef`], if any.
  /// Note: This only provides the basic ObjectRef, not the OwnedObjectRef.
  /// If ownership information is needed, the implementing struct's specific method
  /// should be used, or this trait could be extended.
  async fn get_basic_object_ref_by_id(&self, obj_id: ObjectID) -> Result<Option<ObjectRef>, Error>;
}
