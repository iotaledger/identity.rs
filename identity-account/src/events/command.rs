// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;
use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::IotaDID;

use crate::account::Account;
use crate::error::Result;
use crate::events::CommandError;
use crate::events::Context;
use crate::events::Event;
use crate::events::EventData;
use crate::identity::IdentityId;
use crate::identity::IdentityKey;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;

// Supported authentication method types.
const AUTH_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

#[derive(Clone, Debug)]
pub(crate) enum Command {
  CreateIdentity {
    network: Option<String>,
    method_secret: Option<MethodSecret>,
    authentication: MethodType,
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
    endpoint: Url,
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
        authentication,
      } => {
        // The state must not be initialized
        ensure!(state.did().is_none(), CommandError::DocumentAlreadyExists);

        // The authentication method type must be valid
        ensure!(
          AUTH_TYPES.contains(&authentication),
          CommandError::InvalidMethodType(authentication)
        );

        let generation: Generation = state.auth_generation();
        let location: KeyLocation = KeyLocation::new_authentication(authentication, generation);

        // The key location must be available
        // TODO: config: strict
        ensure!(
          !store.key_exists(state.id(), &location).await?,
          CommandError::DuplicateKeyLocation(location)
        );

        let public: PublicKey = if let Some(method_secret_key) = method_secret {
          insert_method_secret(store, state.id(), &location, authentication, method_secret_key).await
        } else {
          store.key_new(state.id(), &location).await
        }?;

        let data: MethodData = MethodData::new_b58(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data, None);

        // Generate a new DID URL from the public key
        let network: Option<&str> = network.as_deref();
        let document: IotaDID = IotaDID::from_components(public.as_ref(), network)?;

        Ok(Some(vec![
          Event::new(EventData::IdentityCreated(document)),
          // TODO: MethodScope::VerificationMethod when possible
          Event::new(EventData::MethodCreated(MethodScope::Authentication, method)),
        ]))
      }
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        let location: KeyLocation = state.key_location(type_, fragment)?;

        // The key location must not be an authentication location
        ensure!(
          !location.is_authentication(),
          CommandError::InvalidMethodFragment("reserved")
        );

        // The key location must be available
        // TODO: config: strict
        ensure!(
          !store.key_exists(state.id(), &location).await?,
          CommandError::DuplicateKeyLocation(location)
        );

        // The verification method must not exist
        ensure!(
          !state.methods().contains(location.fragment()),
          CommandError::DuplicateKeyFragment(location.fragment.clone()),
        );

        let public: PublicKey = if let Some(method_secret_key) = method_secret {
          insert_method_secret(store, state.id(), &location, type_, method_secret_key).await
        } else {
          store.key_new(state.id(), &location).await
        }?;

        let data: MethodData = MethodData::new_b58(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data, None);

        Ok(Some(vec![Event::new(EventData::MethodCreated(scope, method))]))
      }
      Self::DeleteMethod { fragment } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          CommandError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), CommandError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodDeleted(fragment))]))
      }
      Self::AttachMethod { fragment, scopes } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          CommandError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), CommandError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodAttached(fragment, scopes))]))
      }
      Self::DetachMethod { fragment, scopes } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          CommandError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), CommandError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodDetached(fragment, scopes))]))
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        // The service must not exist
        ensure!(
          !state.services().contains(&fragment),
          CommandError::DuplicateServiceFragment(fragment),
        );

        let service: TinyService = TinyService::new(fragment, type_, endpoint, properties);

        Ok(Some(vec![Event::new(EventData::ServiceCreated(service))]))
      }
      Self::DeleteService { fragment } => {
        // The state must be initialized
        ensure!(state.did().is_some(), CommandError::DocumentNotFound);

        let fragment: Fragment = Fragment::new(fragment);

        // The service must exist
        ensure!(
          state.services().contains(fragment.name()),
          CommandError::ServiceNotFound
        );

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
    MethodSecret::Ed25519(secret_key) => {
      ensure!(
        secret_key.as_ref().len() == ed25519::SECRET_KEY_LENGTH,
        CommandError::InvalidMethodSecret(format!(
          "an ed25519 secret key requires {} bytes, found {}",
          ed25519::SECRET_KEY_LENGTH,
          secret_key.as_ref().len()
        ))
      );

      ensure!(
        matches!(method_type, MethodType::Ed25519VerificationKey2018),
        CommandError::InvalidMethodSecret(
          "MethodType::Ed25519VerificationKey2018 can only be used with an ed25519 method secret".to_owned(),
        )
      );

      store.key_insert(identity_id, location, secret_key).await
    }
    MethodSecret::MerkleKeyCollection(_) => {
      ensure!(
        matches!(method_type, MethodType::MerkleKeyCollection2021),
        CommandError::InvalidMethodSecret(
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

impl_command_builder!(CreateMethod {
  @defaulte type_ MethodType = Ed25519VerificationKey2018,
  @default scope MethodScope,
  @required fragment String,
  @optional method_secret MethodSecret
});

impl_command_builder!(DeleteMethod {
  @required fragment String,
});

impl_command_builder!(AttachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl<'account, 'key, K: IdentityKey> AttachMethodBuilder<'account, 'key, K> {
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scopes.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(DetachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl<'account, 'key, K: IdentityKey> DetachMethodBuilder<'account, 'key, K> {
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scopes.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(CreateService {
  @required fragment String,
  @required type_ String,
  @required endpoint Url,
  @optional properties Object,
});

impl_command_builder!(DeleteService {
  @required fragment String,
});
