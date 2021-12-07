// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_did::did::CoreDIDUrl;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::IotaDID;
use identity_iota::did::IotaDIDUrl;
use identity_iota::document::IotaDocument;
use identity_iota::document::IotaVerificationMethod;

use crate::account::Account;
use crate::error::Result;
use crate::identity::DIDLease;
use crate::identity::IdentitySetup;
use crate::identity::IdentityState;
use crate::storage::Storage;
use crate::types::Generation;
use crate::types::KeyLocation;
use crate::types::MethodSecret;
use crate::updates::UpdateError;

pub const DEFAULT_UPDATE_METHOD_PREFIX: &str = "sign-";

pub(crate) async fn create_identity(setup: IdentitySetup, store: &dyn Storage) -> Result<(DIDLease, IdentityState)> {
  let method_type = match setup.key_type {
    KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
  };

  // The method type must be able to sign document updates.
  ensure!(
    IotaDocument::is_signing_method_type(method_type),
    UpdateError::InvalidMethodType(method_type)
  );

  let generation = Generation::new();
  let fragment: String = format!("{}{}", DEFAULT_UPDATE_METHOD_PREFIX, generation.to_u32());

  let location: KeyLocation = KeyLocation::new(method_type, fragment, generation);

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

  // Generate a new DID from the public key
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

  let public: PublicKey =
    insert_method_secret(store, &did, &location, method_type, MethodSecret::Ed25519(private_key)).await?;

  let method_fragment = location.fragment().to_owned();

  let method: IotaVerificationMethod =
    IotaVerificationMethod::from_did(did, setup.key_type, &public, method_fragment.name())?;

  let document = IotaDocument::from_verification_method(method)?;

  let mut state = IdentityState::new(document);

  // Store the generations at which the method was added
  state.store_method_generations(method_fragment);

  Ok((did_lease, state))
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
}

impl Update {
  pub(crate) async fn process(self, did: &IotaDID, state: &mut IdentityState, storage: &dyn Storage) -> Result<()> {
    debug!("[Update::process] Update = {:?}", self);
    trace!("[Update::process] State = {:?}", state);
    trace!("[Update::process] Store = {:?}", storage);

    match self {
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        let location: KeyLocation = state.key_location(type_, fragment)?;

        // The key location must be available.
        // TODO: config: strict
        ensure!(
          !storage.key_exists(did, &location).await?,
          UpdateError::DuplicateKeyLocation(location)
        );

        let method_url: CoreDIDUrl = did.as_ref().to_url().join(location.fragment().identifier())?;

        if state.document().resolve_method(method_url).is_some() {
          return Err(crate::Error::DIDError(identity_did::Error::MethodAlreadyExists));
        }

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(storage, did, &location, type_, method_private_key).await
        } else {
          storage.key_new(did, &location).await
        }?;

        let method: IotaVerificationMethod =
          IotaVerificationMethod::from_did(did.to_owned(), KeyType::Ed25519, &public, location.fragment().name())?;

        state.store_method_generations(location.fragment().clone());

        state.document_mut().insert_method(method, scope)?;
      }
      Self::DeleteMethod { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;
        let core_method_url: CoreDIDUrl = CoreDIDUrl::from(method_url.clone());

        // Prevent deleting the last method capable of signing the DID document.
        let capability_invocation_set = state.document().as_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &core_method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        state.document_mut().remove_method(method_url)?;
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
            .attach_method_relationship(method_url.clone(), relationship)?;
        }
      }
      Self::DetachMethodRelationship {
        fragment,
        relationships,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;
        let core_method_url: CoreDIDUrl = CoreDIDUrl::from(method_url.clone());

        // Prevent detaching the last method capable of signing the DID document.
        let capability_invocation_set = state.document().as_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &core_method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        for relationship in relationships {
          // Ignore result: detaching is idempotent.
          let _ = state
            .document_mut()
            .detach_method_relationship(method_url.clone(), relationship)?;
        }
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        let fragment = Fragment::new(fragment);
        let did_url: CoreDIDUrl = did.as_ref().to_owned().join(fragment.identifier())?;

        // The service must not exist
        ensure!(
          state.document().service().query(&did_url).is_none(),
          UpdateError::DuplicateServiceFragment(fragment.name().to_owned()),
        );

        let service: Service = Service::builder(properties.unwrap_or_default())
          .id(did_url)
          .service_endpoint(endpoint)
          .type_(type_)
          .build()?;

        state.document_mut().insert_service(service);
      }
      Self::DeleteService { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);
        let service_url = did.to_url().join(fragment.identifier())?;

        // The service must exist
        ensure!(
          state.document().service().query(&service_url).is_some(),
          UpdateError::ServiceNotFound
        );

        state.document_mut().remove_service(service_url)?;
      }
    }

    state.document_mut().set_updated(Timestamp::now_utc());

    Ok(())
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

      todo!("[Update::CreateMethod] Handle MerkleKeyCollection")
    }
  }
}

// =============================================================================
// Update Builders
// =============================================================================

impl_update_builder!(
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

impl<'account> AttachMethodRelationshipBuilder<'account> {
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

impl<'account> DetachMethodRelationshipBuilder<'account> {
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
