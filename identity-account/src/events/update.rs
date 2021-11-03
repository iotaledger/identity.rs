// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use crypto::signatures::ed25519;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_did::did::CoreDIDUrl;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::document::DocumentBuilder;
use identity_did::verifiable::Properties as VerifiableProperties;
use identity_did::verification::MethodData;
use identity_did::verification::MethodRef as CoreMethodRef;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDIDUrl;
use identity_iota::did::IotaDocument;
use identity_iota::did::Properties as BaseProperties;
use identity_iota::tangle::NetworkName;

use crate::account::Account;
use crate::error::Result;
use crate::events::Context;
use crate::events::Event;
use crate::events::EventData;
use crate::events::UpdateError;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentityState;
use crate::identity::TinyMethod;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;

type Properties = VerifiableProperties<BaseProperties>;
type BaseDocument = CoreDocument<Properties, Object, Object>;

// Supported authentication method types.
const AUTH_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

fn key_to_method(type_: KeyType) -> MethodType {
  match type_ {
    KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
  }
}

pub(crate) async fn create_identity(
  setup: IdentitySetup,
  store: &dyn Storage,
) -> Result<(IotaDID, DIDLease, IdentityState)> {
  let authentication = key_to_method(setup.key_type);

  // The authentication method type must be valid
  ensure!(
    AUTH_TYPES.contains(&authentication),
    UpdateError::InvalidMethodType(authentication)
  );

  // TODO: Consider passing in integration_generation and use it to construct state, to assert they are equal.
  let location: KeyLocation = KeyLocation::new_authentication(authentication, Generation::new());

  let keypair: KeyPair = if let Some(MethodSecret::Ed25519(private_key)) = &setup.method_secret {
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
  let did: IotaDID = if let Some(network) = &setup.network {
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
    authentication,
    MethodSecret::Ed25519(private_key),
  )
  .await?;

  let data: MethodData = MethodData::new_b58(public.as_ref());

  let method_fragment = location.fragment().to_owned();
  let method: VerificationMethod = core_method(location.method(), data, &did, location.fragment.clone())?;
  let method_ref: CoreMethodRef = core_method_ref(&did, location.fragment)?;

  let properties: BaseProperties = BaseProperties::new();
  let properties: Properties = VerifiableProperties::new(properties);

  let mut builder: DocumentBuilder<_, _, _> = BaseDocument::builder(properties);

  builder = builder
    .id(did.clone().into())
    .verification_method(method)
    .capability_invocation(method_ref);

  let document: IotaDocument = builder.build()?.try_into()?;

  let mut state = IdentityState::new(document);

  // Store the generations at which the method was added
  state.insert_method_location(method_fragment);

  Ok((did.clone(), did_lease, state))
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
          !state.has_method(&location.fragment),
          UpdateError::DuplicateKeyFragment(location.fragment.clone()),
        );

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(store, did, &location, type_, method_private_key).await
        } else {
          store.key_new(did, &location).await
        }?;

        let data: MethodData = MethodData::new_multibase(public.as_ref());
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
        ensure!(state.has_method(&fragment), UpdateError::MethodNotFound);

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
        ensure!(state.has_method(&fragment), UpdateError::MethodNotFound);

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
        ensure!(state.has_method(&fragment), UpdateError::MethodNotFound);

        Ok(Some(vec![Event::new(EventData::MethodDetached(fragment, scopes))]))
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        let did_url = did.to_url().join(fragment.clone())?;
        // The service must not exist
        ensure!(
          state.as_document().service().query(&did_url).is_none(),
          UpdateError::DuplicateServiceFragment(fragment),
        );

        let service: TinyService = TinyService::new(fragment, type_, endpoint, properties);

        Ok(Some(vec![Event::new(EventData::ServiceCreated(service))]))
      }
      Self::DeleteService { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);
        let service_url = did.to_url().join(fragment.name())?;

        // The service must exist
        ensure!(
          state.as_document().service().query(&service_url).is_some(),
          UpdateError::ServiceNotFound
        );

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

fn core_method(
  method_type: MethodType,
  method_data: MethodData,
  did: &IotaDID,
  fragment: Fragment,
) -> Result<VerificationMethod> {
  let id: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

  VerificationMethod::builder(Object::default())
    .id(CoreDIDUrl::from(id))
    .controller(did.clone().into())
    .key_type(method_type)
    .key_data(method_data)
    .build()
    .map_err(Into::into)
}

fn core_method_ref(did: &IotaDID, fragment: Fragment) -> Result<CoreMethodRef> {
  // TODO: Can return a fatal error here, since the fragment we pass in
  // is always valid, as is the url.
  did
    .to_url()
    .join(fragment.identifier())
    .map(CoreDIDUrl::from)
    .map(CoreMethodRef::Refer)
    .map_err(Into::into)
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
