// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use crypto::signatures::ed25519;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Timestamp;
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
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDIDUrl;
use identity_iota::did::IotaDocument;
use identity_iota::did::IotaVerificationMethod;
use identity_iota::did::Properties as BaseProperties;

use crate::account::Account;
use crate::error::Result;
use crate::events::Event;
use crate::events::UpdateError;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentityState;
use crate::identity::TinyService;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;

// Method types allowed to sign a DID document update.
pub const UPDATE_METHOD_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];
pub const DEFAULT_UPDATE_METHOD_PREFIX: &str = "sign-";

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

  // The method type must be able to sign document updates.
  ensure!(
    UPDATE_METHOD_TYPES.contains(&setup.method_type),
    UpdateError::InvalidMethodType(setup.method_type)
  );

  let integration_generation = Generation::new();
  let fragment: String = format!("{}{}", DEFAULT_UPDATE_METHOD_PREFIX, integration_generation.to_u32());
  // TODO: Consider passing in integration_generation and use it to construct state, to assert they are equal.

  let location: KeyLocation = KeyLocation::new(setup.method_type, fragment, integration_generation, Generation::new());

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

  // TODO: Embed capability invocation
  builder = builder
    .id(did.clone().into())
    .verification_method(method)
    .capability_invocation(method_ref);

  let document: IotaDocument = builder.build()?.try_into()?;

  let mut state = IdentityState::new(document);

  // Store the generations at which the method was added
  state.set_method_generations(method_fragment);

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
    relationships: Vec<MethodRelationship>,
  },
  DetachMethod {
    fragment: String,
    relationships: Vec<MethodRelationship>,
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
  pub(crate) async fn process(
    self,
    did: &IotaDID,
    state: &mut IdentityState,
    storage: &dyn Storage,
  ) -> Result<Option<Vec<Event>>> {
    debug!("[Command::process] Command = {:?}", self);
    trace!("[Command::process] State = {:?}", state);
    trace!("[Command::process] Store = {:?}", storage);

    match self {
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        // TODO: Remove clone after merge
        let location: KeyLocation = state.key_location(type_, fragment.clone())?;

        // The key location must be available.
        // TODO: config: strict
        ensure!(
          !storage.key_exists(did, &location).await?,
          UpdateError::DuplicateKeyLocation(location)
        );

        // The verification method must not exist.
        ensure!(
          state.as_document().resolve(location.fragment().identifier()).is_none(),
          UpdateError::DuplicateKeyFragment(location.fragment().clone()),
        );

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(storage, did, &location, type_, method_private_key).await
        } else {
          storage.key_new(did, &location).await
        }?;

        // TODO: Fix after merge
        // let opt_fragment: String = fragment.clone();
        let opt_fragment_str: Option<&str> = Some(&fragment);
        let method: IotaVerificationMethod =
          IotaVerificationMethod::from_did(did.to_owned(), KeyType::Ed25519, &public, opt_fragment_str)?;

        state.set_method_generations(fragment);

        // We can ignore the result: we just checked that the method does not exist.
        state.as_document_mut().insert_method(scope, method);
      }
      Self::DeleteMethod { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);

        // The verification method must exist
        ensure!(
          state.as_document().resolve(fragment.identifier()).is_some(),
          UpdateError::MethodNotFound
        );

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

        // TODO: Do we have to ? here?
        let method_url = did.to_url().join(fragment.identifier())?;

        // TODO: Still fallible after merge?
        state.as_document_mut().remove_method(method_url);
      }
      Self::AttachMethod {
        fragment,
        relationships,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        // TODO: Do we have to ? here?
        let method_url = did.to_url().join(fragment.identifier())?;

        // The verification method must exist
        ensure!(
          state.as_document().resolve(fragment.identifier()).is_some(),
          UpdateError::MethodNotFound
        );

        for relationship in relationships {
          // We ignore the boolean result: if the relationship already existed, that's fine.
          state
            .as_document_mut()
            .attach_method_relationship(method_url.clone(), relationship)
            .map_err(|_| UpdateError::MethodNotFound)?;
        }
      }
      Self::DetachMethod {
        fragment,
        relationships,
      } => {
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

        // TODO: Do we have to ? here?
        let method_url = did.to_url().join(fragment.identifier())?;

        for relationship in relationships {
          state
            .as_document_mut()
            .detach_method_relationship(method_url.clone(), relationship)
            .map_err(|_| UpdateError::MethodNotFound)?;
        }

        // Ok(Some(vec![Event::new(EventData::MethodDetached(fragment, scopes))]))
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

        // Ok(Some(vec![Event::new(EventData::ServiceCreated(service))]))
      }
      Self::DeleteService { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);
        let service_url = did.to_url().join(fragment.name())?;

        // The service must exist
        ensure!(
          state.as_document().service().query(&service_url).is_some(),
          UpdateError::ServiceNotFound
        );

        // Ok(Some(vec![Event::new(EventData::ServiceDeleted(fragment))]))
      }
    }

    state.as_document_mut().set_updated(Timestamp::now_utc());

    Ok(None)
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
/// - `relationships`: the relationships to add, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
AttachMethod {
  @required fragment String,
  @default relationships Vec<MethodRelationship>,
});

impl<'account> AttachMethodBuilder<'account> {
  pub fn relationship(mut self, value: MethodRelationship) -> Self {
    self.relationships.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_command_builder!(
/// Detaches one or more verification relationships from a method on an identity.
///
/// # Parameters
/// - `relationships`: the relationships to remove, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
DetachMethod {
  @required fragment String,
  @default relationships Vec<MethodRelationship>,
});

impl<'account> DetachMethodBuilder<'account> {
  pub fn relationship(mut self, value: MethodRelationship) -> Self {
    self.relationships.get_or_insert_with(Default::default).push(value);
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
