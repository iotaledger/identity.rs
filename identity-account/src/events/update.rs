// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
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
use crate::identity::DIDLease;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;

// Supported authentication method types.
const AUTH_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

pub(crate) struct CreateIdentity {
  pub(crate) network: Option<NetworkName>,
  pub(crate) method_secret: Option<MethodSecret>,
  pub(crate) authentication: MethodType,
}

impl CreateIdentity {
  pub(crate) async fn process(
    &self,
    integration_generation: Generation,
    store: &dyn Storage,
  ) -> Result<(IotaDID, DIDLease, Vec<Event>)> {
    // The authentication method type must be valid
    ensure!(
      AUTH_TYPES.contains(&self.authentication),
      UpdateError::InvalidMethodType(self.authentication)
    );

    let location: KeyLocation = KeyLocation::new_authentication(self.authentication, integration_generation);

    let keypair: KeyPair = if let Some(MethodSecret::Ed25519(private_key)) = &self.method_secret {
      ensure!(
        private_key.as_ref().len() == ed25519::SECRET_KEY_LENGTH,
        UpdateError::InvalidMethodSecret(format!(
          "an ed25519 private key requires {} bytes, found {}",
          ed25519::SECRET_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );

      KeyPair::try_from_ed25519_bytes(private_key.as_ref())?
    } else {
      KeyPair::new_ed25519()?
    };

    // Generate a new DID URL from the public key
    let did: IotaDID = if let Some(network) = &self.network {
      IotaDID::new_with_network(keypair.public().as_ref(), network.clone())?
    } else {
      IotaDID::new(keypair.public().as_ref())?
    };

    ensure!(
      !store.key_exists(&did, &location).await?,
      UpdateError::DocumentAlreadyExists
    );

    let did_lease = store.lease_did(&did).await?;

    let private_key = keypair.private().to_owned();
    std::mem::drop(keypair);

    let public: PublicKey = insert_method_secret(
      store,
      &did,
      &location,
      self.authentication,
      MethodSecret::Ed25519(private_key),
    )
    .await?;

    let data: MethodData = MethodData::new_b58(public.as_ref());
    let method: TinyMethod = TinyMethod::new(location, data, None);

    let method_fragment = Fragment::new(method.location().fragment());

    Ok((
      did.clone(),
      did_lease,
      vec![
        Event::new(EventData::IdentityCreated(did)),
        Event::new(EventData::MethodCreated(MethodScope::VerificationMethod, method)),
        Event::new(EventData::MethodAttached(
          method_fragment,
          vec![MethodScope::Authentication],
        )),
      ],
    ))
  }
}

#[derive(Clone, Debug)]
pub(crate) enum Update {
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

impl Update {
  pub(crate) async fn process(self, context: Context<'_>) -> Result<Option<Vec<Event>>> {
    let did: &IotaDID = context.did();
    let state: &IdentityState = context.state();
    let store: &dyn Storage = context.store();

    debug!("[Command::process] Command = {:?}", self);
    trace!("[Command::process] State = {:?}", state);
    trace!("[Command::process] Store = {:?}", store);

    match self {
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        let location: KeyLocation = state.key_location(type_, fragment)?;

        // The key location must not be an authentication location
        ensure!(
          !location.is_authentication(),
          UpdateError::InvalidMethodFragment("reserved")
        );

        // The key location must be available
        // TODO: config: strict
        ensure!(
          !store.key_exists(did, &location).await?,
          UpdateError::DuplicateKeyLocation(location)
        );

        // The verification method must not exist
        ensure!(
          !state.methods().contains(location.fragment()),
          UpdateError::DuplicateKeyFragment(location.fragment.clone()),
        );

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(store, did, &location, type_, method_private_key).await
        } else {
          store.key_new(did, &location).await
        }?;

        let data: MethodData = MethodData::new_b58(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data, None);

        Ok(Some(vec![Event::new(EventData::MethodCreated(scope, method))]))
      }
      Self::DeleteMethod { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          UpdateError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodDeleted(fragment))]))
      }
      Self::AttachMethod { fragment, scopes } => {
        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          UpdateError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodAttached(fragment, scopes))]))
      }
      Self::DetachMethod { fragment, scopes } => {
        let fragment: Fragment = Fragment::new(fragment);

        // The fragment must not be an authentication location
        ensure!(
          !KeyLocation::is_authentication_fragment(&fragment),
          UpdateError::InvalidMethodFragment("reserved")
        );

        // The verification method must exist
        ensure!(state.methods().contains(fragment.name()), UpdateError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodDetached(fragment, scopes))]))
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        // The service must not exist
        ensure!(
          !state.services().contains(&fragment),
          UpdateError::DuplicateServiceFragment(fragment),
        );

        let service: TinyService = TinyService::new(fragment, type_, endpoint, properties);

        Ok(Some(vec![Event::new(EventData::ServiceCreated(service))]))
      }
      Self::DeleteService { fragment } => {
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
  did: &IotaDID,
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

      store.key_insert(did, location, private_key).await
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

impl<'account> AttachMethodBuilder<'account> {
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

impl<'account> DetachMethodBuilder<'account> {
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
/// - `endpoint`: the url of the service, required.
/// - `properties`: additional properties of the service, optional.
CreateService {
  @required fragment String,
  @required type_ String,
  @required endpoint Url,
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
