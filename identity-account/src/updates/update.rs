// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use log::debug;
use log::trace;

use identity_account_storage::identity::IdentityState;
use identity_account_storage::storage::Storage;
use identity_account_storage::types::Generation;
use identity_account_storage::types::KeyLocation;
use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::Ed25519;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_core::crypto::X25519;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use identity_did::utils::Queryable;
use identity_did::verification::MethodRef;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::tangle::Client;
use identity_iota::tangle::SharedPtr;
use identity_iota_core::did::IotaDID;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaDocument;
use identity_iota_core::document::IotaService;
use identity_iota_core::document::IotaVerificationMethod;
use identity_iota_core::tangle::NetworkName;

use crate::account::Account;
use crate::error::Result;
use crate::types::IdentitySetup;
use crate::types::MethodContent;
use crate::updates::UpdateError;

pub const DEFAULT_UPDATE_METHOD_PREFIX: &str = "sign-";

pub(crate) async fn create_identity(
  setup: IdentitySetup,
  network: NetworkName,
  store: &dyn Storage,
) -> Result<IdentityState> {
  let keypair: KeyPair = match setup.private_key {
    None => KeyPair::new(KeyType::Ed25519)?,
    Some(private_key) => {
      ensure!(
        private_key.as_ref().len() == Ed25519::PRIVATE_KEY_LENGTH,
        UpdateError::InvalidMethodContent(format!(
          "an Ed25519 private key requires {} bytes, found {}",
          Ed25519::PRIVATE_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );
      KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())?
    }
  };

  // Generate a new DID from the public key
  let did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network)?;

  // Store the private key.
  let generation: Generation = Generation::new();
  let fragment: String = format!("{}{}", DEFAULT_UPDATE_METHOD_PREFIX, generation.to_u32());
  let location: KeyLocation = KeyLocation::new(MethodType::Ed25519VerificationKey2018, fragment, generation);
  ensure!(
    !store.key_exists(&did, &location).await?,
    UpdateError::DocumentAlreadyExists
  );
  let private_key: PrivateKey = keypair.private().to_owned();
  std::mem::drop(keypair);
  let public_key: PublicKey = insert_method_secret(store, &did, &location, private_key).await?;

  // Construct a new DID Document.
  let method_fragment: Fragment = location.fragment().to_owned();
  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(did, KeyType::Ed25519, &public_key, method_fragment.name())?;

  let document: IotaDocument = IotaDocument::from_verification_method(method)?;
  let mut state: IdentityState = IdentityState::new(document);

  // Store the generations at which the method was added
  state.store_method_generations(method_fragment);

  Ok(state)
}

#[derive(Clone, Debug)]
pub(crate) enum Update {
  CreateMethod {
    scope: MethodScope,
    content: MethodContent,
    fragment: String,
  },
  DeleteMethod {
    fragment: String,
  },
  AttachMethodRelationship {
    fragment: String,
    relationships: Vec<MethodRelationship>,
  },
  DetachMethodRelationship {
    fragment: String,
    relationships: Vec<MethodRelationship>,
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
  SetController {
    controllers: Option<OneOrSet<IotaDID>>,
  },
  SetAlsoKnownAs {
    urls: OrderedSet<Url>,
  },
}

impl Update {
  pub(crate) async fn process(self, did: &IotaDID, state: &mut IdentityState, storage: &dyn Storage) -> Result<()> {
    debug!("[Update::process] Update = {:?}", self);
    trace!("[Update::process] State = {:?}", state);
    trace!("[Update::process] Store = {:?}", storage);

    match self {
      Self::CreateMethod {
        scope,
        content,
        fragment,
      } => {
        let method_fragment: String = if !fragment.starts_with('#') {
          format!("#{}", fragment)
        } else {
          fragment
        };

        // Check method identifier is not duplicated.
        let method_url: IotaDIDUrl = did.to_url().join(&method_fragment)?;
        if state.document().resolve_method(method_url, None).is_some() {
          return Err(crate::Error::DIDError(identity_did::Error::MethodAlreadyExists));
        }

        // Generate or extract the private key and/or retrieve the public key.
        let key_type: KeyType = content.key_type();
        let method_type: MethodType = content.method_type();
        let public: PublicKey = match content {
          MethodContent::GenerateEd25519 => {
            let location: KeyLocation =
              create_method_key_location(did, storage, state, method_type, method_fragment.clone()).await?;
            storage.key_new(did, &location).await?
          }
          MethodContent::PrivateEd25519(private_key) => {
            let location: KeyLocation =
              create_method_key_location(did, storage, state, method_type, method_fragment.clone()).await?;
            insert_method_secret(storage, did, &location, private_key).await?
          }
          MethodContent::PublicEd25519(public_key) => public_key,
          MethodContent::GenerateX25519 => {
            let location: KeyLocation =
              create_method_key_location(did, storage, state, method_type, method_fragment.clone()).await?;
            storage.key_new(did, &location).await?
          }
          MethodContent::PrivateX25519(private_key) => {
            let location: KeyLocation =
              create_method_key_location(did, storage, state, method_type, method_fragment.clone()).await?;
            insert_method_secret(storage, did, &location, private_key).await?
          }
          MethodContent::PublicX25519(public_key) => public_key,
        };

        // Insert a new method.
        let method: IotaVerificationMethod =
          IotaVerificationMethod::new(did.clone(), key_type, &public, &method_fragment)?;
        state.document_mut().insert_method(method, scope)?;
      }
      Self::DeleteMethod { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // Prevent deleting the last method capable of signing the DID document.
        let capability_invocation_set = state.document().core_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        state.document_mut().remove_method(&method_url)?;
      }
      Self::AttachMethodRelationship {
        fragment,
        relationships,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        for relationship in relationships {
          // Ignore result: attaching is idempotent.
          let _ = state
            .document_mut()
            .attach_method_relationship(&method_url, relationship)?;
        }
      }
      Self::DetachMethodRelationship {
        fragment,
        relationships,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // Prevent detaching the last method capable of signing the DID document.
        let capability_invocation_set: &OrderedSet<MethodRef<IotaDID>> =
          state.document().core_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        for relationship in relationships {
          // Ignore result: detaching is idempotent.
          let _ = state
            .document_mut()
            .detach_method_relationship(&method_url, relationship)?;
        }
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        let fragment = Fragment::new(fragment);
        let did_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // The service must not exist.
        ensure!(
          state.document().service().query(&did_url).is_none(),
          UpdateError::DuplicateServiceFragment(fragment.name().to_owned()),
        );

        let service: IotaService = Service::builder(properties.unwrap_or_default())
          .id(did_url)
          .service_endpoint(endpoint)
          .type_(type_)
          .build()?;

        state.document_mut().insert_service(service);
      }
      Self::DeleteService { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);
        let service_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // The service must exist
        ensure!(
          state.document().service().query(&service_url).is_some(),
          UpdateError::ServiceNotFound
        );

        state.document_mut().remove_service(&service_url)?;
      }
      Self::SetController { controllers } => {
        *state.document_mut().controller_mut() = controllers;
      }

      Self::SetAlsoKnownAs { urls } => {
        *state.document_mut().also_known_as_mut() = urls;
      }
    }

    state.document_mut().metadata.updated = Timestamp::now_utc();

    Ok(())
  }
}

/// Helper function to create a new key location and ensure it is available.
async fn create_method_key_location(
  did: &IotaDID,
  storage: &dyn Storage,
  state: &mut IdentityState,
  method_type: MethodType,
  method_fragment: String,
) -> Result<KeyLocation> {
  let location: KeyLocation = state.key_location(method_type, method_fragment)?;
  ensure!(
    !storage.key_exists(did, &location).await?,
    UpdateError::DuplicateKeyLocation(location)
  );
  state.store_method_generations(location.fragment().clone());
  Ok(location)
}

async fn insert_method_secret(
  store: &dyn Storage,
  did: &IotaDID,
  location: &KeyLocation,
  private_key: PrivateKey,
) -> Result<PublicKey> {
  match location.method() {
    MethodType::Ed25519VerificationKey2018 => {
      ensure!(
        private_key.as_ref().len() == Ed25519::PRIVATE_KEY_LENGTH,
        UpdateError::InvalidMethodContent(format!(
          "an ed25519 private key requires {} bytes, got {}",
          Ed25519::PRIVATE_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );
      store.key_insert(did, location, private_key).await.map_err(Into::into)
    }
    MethodType::X25519KeyAgreementKey2019 => {
      ensure!(
        private_key.as_ref().len() == X25519::PRIVATE_KEY_LENGTH,
        UpdateError::InvalidMethodContent(format!(
          "an x25519 private key requires {} bytes, got {}",
          X25519::PRIVATE_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );
      store.key_insert(did, location, private_key).await.map_err(Into::into)
    }
  }
}

// =============================================================================

// =============================================================================
// Update Builders
impl_update_builder!(
/// Create a new method on an identity.
///
/// # Parameters
/// - `scope`: the scope of the method, defaults to [`MethodScope::default`].
/// - `fragment`: the identifier of the method in the document, required.
/// - `content`: the key material to use for the method or key type to generate.
CreateMethod {
  @default scope MethodScope,
  @required fragment String,
  @required content MethodContent,
});

impl_update_builder!(
/// Delete a method on an identity.
///
/// # Parameters
/// - `fragment`: the identifier of the method in the document, required.
DeleteMethod {
  @required fragment String,
});

impl_update_builder!(
/// Attach one or more verification relationships to a method on an identity.
///
/// # Parameters
/// - `relationships`: the relationships to add, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
AttachMethodRelationship {
  @required fragment String,
  @default relationships Vec<MethodRelationship>,
});

impl<'account, C> AttachMethodRelationshipBuilder<'account, C>
where
  C: SharedPtr<Client>,
{
  #[must_use]
  pub fn relationship(mut self, value: MethodRelationship) -> Self {
    self.relationships.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_update_builder!(
/// Detaches one or more verification relationships from a method on an identity.
///
/// # Parameters
/// - `relationships`: the relationships to remove, defaults to an empty [`Vec`].
/// - `fragment`: the identifier of the method in the document, required.
DetachMethodRelationship {
  @required fragment String,
  @default relationships Vec<MethodRelationship>,
});

impl<'account, C> DetachMethodRelationshipBuilder<'account, C>
where
  C: SharedPtr<Client>,
{
  #[must_use]
  pub fn relationship(mut self, value: MethodRelationship) -> Self {
    self.relationships.get_or_insert_with(Default::default).push(value);
    self
  }
}

impl_update_builder!(
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

impl_update_builder!(
/// Delete a service on an identity.
///
/// # Parameters
/// - `fragment`: the identifier of the service in the document, required.
DeleteService {
  @required fragment String,
});

impl_update_builder!(
SetController {
    @required controllers Option<OneOrSet<IotaDID>>,
});

impl_update_builder!(
SetAlsoKnownAs {
    @required urls OrderedSet<Url>,
});
