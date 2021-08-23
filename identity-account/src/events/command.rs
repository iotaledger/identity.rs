// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::PublicKey;
use identity_core::crypto::SecretKey;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::IotaDID;

use crate::error::Result;
use crate::events::CommandError;
use crate::events::Context;
use crate::events::Event;
use crate::events::EventData;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;

// Supported authentication method types.
const AUTH_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

#[derive(Clone, Debug)]
pub enum Command {
  CreateIdentity {
    network: Option<String>,
    secret_key: Option<SecretKey>,
    authentication: MethodType,
  },
  CreateMethod {
    scope: MethodScope,
    type_: MethodType,
    fragment: String,
    secret_key: Option<SecretKey>,
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
  pub async fn process(self, context: Context<'_>) -> Result<Option<Vec<Event>>> {
    let state: &IdentityState = context.state();
    let store: &dyn Storage = context.store();

    debug!("[Command::process] Command = {:?}", self);
    trace!("[Command::process] State = {:?}", state);
    trace!("[Command::process] Store = {:?}", store);

    match self {
      Self::CreateIdentity {
        network,
        secret_key,
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

        let public: PublicKey = if let Some(secret_key) = secret_key {
          store.key_insert(state.id(), &location, secret_key).await
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
        secret_key,
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

        let public: PublicKey = if let Some(secret_key) = secret_key {
          store.key_insert(state.id(), &location, secret_key).await?
        } else {
          store.key_new(state.id(), &location).await?
        };

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

// =============================================================================
// Command Builders
// =============================================================================

impl_command_builder!(CreateIdentity {
  @optional network String,
  @optional secret_key SecretKey,
  @defaulte authentication MethodType = Ed25519VerificationKey2018,
});

impl_command_builder!(CreateMethod {
  @defaulte type_ MethodType = Ed25519VerificationKey2018,
  @default scope MethodScope,
  @required fragment String,
  @optional secret_key SecretKey
});

impl_command_builder!(DeleteMethod {
  @required fragment String,
});

impl_command_builder!(AttachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl AttachMethodBuilder {
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scopes.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(DetachMethod {
  @required fragment String,
  @default scopes Vec<MethodScope>,
});

impl DetachMethodBuilder {
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
