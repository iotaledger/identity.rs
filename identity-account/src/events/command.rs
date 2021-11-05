// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::crypto::PublicKey;
use identity_did::service::ServiceEndpoint;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::IotaDID;
use identity_iota::tangle::NetworkName;

use crate::account::Account;
use crate::error::Result;
use crate::events::Context;
use crate::events::Event;
use crate::events::EventData;
use crate::events::UpdateError;
use crate::identity::IdentityId;
use crate::identity::IdentityKey;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;

// Method types allowed to sign a DID document update.
pub const UPDATE_METHOD_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];
pub const DEFAULT_UPDATE_METHOD_PREFIX: &str = "sign-";

#[derive(Clone, Debug)]
pub(crate) enum Command {
  CreateIdentity {
    network: Option<NetworkName>,
    method_secret: Option<MethodSecret>,
    method_type: MethodType,
  },
  CreateMethod {
    scope: MethodScope,
    type_: MethodType,
    fragment: String,
    method_secret: Option<MethodSecret>,
  },
  DeleteMethod {
    fragment: String,
  },
  AttachMethod {
    fragment: String,
    scopes: Vec<MethodScope>,
  },
  DetachMethod {
    fragment: String,
    scopes: Vec<MethodScope>,
  },
  CreateService {
    fragment: String,
    type_: String,
    endpoint: ServiceEndpoint,
    properties: Option<Object>,
  },
  DeleteService {
    fragment: String,
  },
}

impl Command {
  pub(crate) async fn process(self, context: Context<'_>) -> Result<Option<Vec<Event>>> {
    let state: &IdentityState = context.state();
    let store: &dyn Storage = context.store();

    debug!("[Command::process] Command = {:?}", self);
    trace!("[Command::process] State = {:?}", state);
    trace!("[Command::process] Store = {:?}", store);

    match self {
      Self::CreateIdentity {
        network,
        method_secret,
        method_type,
      } => {
        // The state must not be initialized.
        ensure!(state.did().is_none(), UpdateError::DocumentAlreadyExists);

        // The method type must be able to sign document updates.
        ensure!(
          UPDATE_METHOD_TYPES.contains(&method_type),
          UpdateError::InvalidMethodType(method_type)
        );

        let generation: Generation = state.integration_generation();
        let fragment: String = format!("{}{}", DEFAULT_UPDATE_METHOD_PREFIX, generation.to_u32());
        let location: KeyLocation = KeyLocation::new(method_type, fragment, generation, Generation::new());

        // The key location must be available.
        // TODO: config: strict
        ensure!(
          !store.key_exists(state.id(), &location).await?,
          UpdateError::DuplicateKeyLocation(location)
        );

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(store, state.id(), &location, method_type, method_private_key).await
        } else {
          store.key_new(state.id(), &location).await
        }?;

        let data: MethodData = MethodData::new_multibase(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data, None);

        // Generate a new DID from the public key.
        let did: IotaDID = if let Some(network) = network {
          IotaDID::new_with_network(public.as_ref(), network)?
        } else {
          IotaDID::new(public.as_ref())?
        };

        Ok(Some(vec![
          Event::new(EventData::IdentityCreated(did)),
          Event::new(EventData::MethodCreated(MethodScope::CapabilityInvocation, method)),
        ]))
      }
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        // The state must be initialized.
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        let location: KeyLocation = state.key_location(type_, fragment)?;

        // The key location must be available.
        // TODO: config: strict
        ensure!(
          !store.key_exists(state.id(), &location).await?,
          UpdateError::DuplicateKeyLocation(location)
        );

        // The verification method must not exist.
        ensure!(
          !state.methods().contains(location.fragment_name()),
          UpdateError::DuplicateKeyFragment(location.fragment().clone()),
        );

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(store, state.id(), &location, type_, method_private_key).await
        } else {
          store.key_new(state.id(), &location).await
        }?;

        let data: MethodData = MethodData::new_multibase(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data, None);

        Ok(Some(vec![Event::new(EventData::MethodCreated(scope, method))]))
      }
      Self::DeleteMethod { fragment } => {
        // The state must be initialized.
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The verification method must exist.
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        // Prevent deleting the last method capable of signing the DID document.
        let is_capability_invocation = state
          .methods()
          .slice(MethodScope::CapabilityInvocation)
          .iter()
          .any(|method_ref| method_ref.fragment() == &fragment);
        ensure!(
          !(is_capability_invocation && state.methods().slice(MethodScope::CapabilityInvocation).len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        Ok(Some(vec![Event::new(EventData::MethodDeleted(fragment))]))
      }
      Self::AttachMethod { fragment, scopes } => {
        // The state must be initialized.
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The verification method must exist.
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodAttached(fragment, scopes))]))
      }
      Self::DetachMethod { fragment, scopes } => {
        // The state must be initialized.
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The verification method must exist.
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        // Prevent detaching the last method capable of signing the DID document.
        let is_capability_invocation = state
          .methods()
          .slice(MethodScope::CapabilityInvocation)
          .iter()
          .any(|method_ref| method_ref.fragment() == &fragment);
        ensure!(
          !(is_capability_invocation && state.methods().slice(MethodScope::CapabilityInvocation).len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        Ok(Some(vec![Event::new(EventData::MethodDetached(fragment, scopes))]))
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        // The state must be initialized
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        // The service must not exist
        ensure!(
          !state.services().contains(&fragment),
          UpdateError::DuplicateServiceFragment(fragment),
        );

        let service: TinyService = TinyService::new(fragment, type_, endpoint, properties);

        Ok(Some(vec![Event::new(EventData::ServiceCreated(service))]))
      }
      Self::DeleteService { fragment } => {
        // The state must be initialized
        ensure!(state.did().is_some(), UpdateError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The service must exist
        ensure!(state.services().contains(fragment.name()), UpdateError::ServiceNotFound);

        Ok(Some(vec![Event::new(EventData::ServiceDeleted(fragment))]))
      }
    }
  }
}

async fn insert_method_secret(
  store: &dyn Storage,
  identity_id: IdentityId,
  location: &KeyLocation,
  method_type: MethodType,
  method_secret: MethodSecret,
) -> Result<PublicKey> {
  match method_secret {
    MethodSecret::Ed25519(private_key) => {
      ensure!(
        private_key.as_ref().len() == ed25519::SECRET_KEY_LENGTH,
        UpdateError::InvalidMethodSecret(format!(
          "an ed25519 private key requires {} bytes, found {}",
          ed25519::SECRET_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );

      ensure!(
        matches!(method_type, MethodType::Ed25519VerificationKey2018),
        UpdateError::InvalidMethodSecret(
          "MethodType::Ed25519VerificationKey2018 can only be used with an ed25519 method secret".to_owned(),
        )
      );

      store.key_insert(identity_id, location, private_key).await
    }
    MethodSecret::MerkleKeyCollection(_) => {
      ensure!(
        matches!(method_type, MethodType::MerkleKeyCollection2021),
        UpdateError::InvalidMethodSecret(
          "MethodType::MerkleKeyCollection2021 can only be used with a MerkleKeyCollection method secret".to_owned(),
        )
      );

      todo!("[Command::CreateMethod] Handle MerkleKeyCollection")
    }
  }
}

// =============================================================================
// Command Builders
// =============================================================================

impl_command_builder!(
/// Create a new method on an identity.
///
/// # Parameters
/// - `type_`: the type of the method, defaults to [`MethodType::Ed25519VerificationKey2018`].
/// - `scope`: the scope of the method, defaults to [`MethodScope::default`].
/// - `fragment`: the identifier of the method in the document, required.
/// - `method_secret`: the secret key to use for the method, optional. Will be generated when omitted.
CreateMethod {
  @defaulte type_ MethodType = Ed25519VerificationKey2018,
  @default scope MethodScope,
  @required fragment String,
  @optional method_secret MethodSecret
});

impl_command_builder!(
/// Delete a method on an identity.
///
/// # Parameters
/// - `fragment`: the identifier of the method in the document, required.
DeleteMethod {
  @required fragment String,
});

impl_command_builder!(
/// Attach one or more verification relationships to a method on an identity.
///
/// # Parameters
/// - `scopes`: the scopes to add, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
AttachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl<'account, 'key, K: IdentityKey> AttachMethodBuilder<'account, 'key, K> {
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scopes.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(
/// Detaches one or more verification relationships from a method on an identity.
///
/// # Parameters
/// - `scopes`: the scopes to remove, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
DetachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl<'account, 'key, K: IdentityKey> DetachMethodBuilder<'account, 'key, K> {
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scopes.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(
/// Create a new service on an identity.
///
/// # Parameters
/// - `type_`: the type of the service, e.g. `"LinkedDomains"`, required.
/// - `fragment`: the identifier of the service in the document, required.
/// - `endpoint`: the `ServiceEndpoint` of the service, required.
/// - `properties`: additional properties of the service, optional.
CreateService {
  @required fragment String,
  @required type_ String,
  @required endpoint ServiceEndpoint,
  @optional properties Object,
});

impl_command_builder!(
/// Delete a service on an identity.
///
/// # Parameters
/// - `fragment`: the identifier of the service in the document, required.
DeleteService {
  @required fragment String,
});
