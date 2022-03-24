// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::x25519;
use crypto::signatures::ed25519;
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
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PublicKey;
use identity_did::did::CoreDIDUrl;
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
use crate::identity::IdentitySetup;
use crate::types::MethodSecret;
use crate::updates::UpdateError;

pub const DEFAULT_UPDATE_METHOD_PREFIX: &str = "sign-";

pub(crate) async fn create_identity(
  setup: IdentitySetup,
  network: NetworkName,
  store: &dyn Storage,
) -> Result<IdentityState> {
  let method_type: MethodType = match setup.key_type {
    KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
    KeyType::X25519 => MethodType::X25519KeyAgreementKey2019,
  };

  // The method type must be able to sign document updates.
  ensure!(
    IotaDocument::is_signing_method_type(method_type),
    UpdateError::InvalidMethodType(method_type)
  );

  let generation = Generation::new();
  let fragment: String = format!("{}{}", DEFAULT_UPDATE_METHOD_PREFIX, generation.to_u32());

  let location: KeyLocation = KeyLocation::new(method_type, fragment, generation);

  let keypair: KeyPair = match &setup.method_secret {
    None => KeyPair::new(KeyType::Ed25519)?,
    Some(MethodSecret::Ed25519(private_key)) => {
      ensure!(
        private_key.as_ref().len() == ed25519::SECRET_KEY_LENGTH,
        UpdateError::InvalidMethodSecret(format!(
          "an Ed25519 private key requires {} bytes, found {}",
          ed25519::SECRET_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );
      KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())?
    }
    _ => {
      return Err(UpdateError::InvalidMethodSecret("expected None or Ed25519 private key".to_owned()).into());
    }
  };

  // Generate a new DID from the public key
  let did: IotaDID = IotaDID::new_with_network(keypair.public().as_ref(), network)?;

  ensure!(
    !store.key_exists(&did, &location).await?,
    UpdateError::DocumentAlreadyExists
  );

  let private_key = keypair.private().to_owned();
  std::mem::drop(keypair);

  let public: PublicKey =
    insert_method_secret(store, &did, &location, method_type, MethodSecret::Ed25519(private_key)).await?;

  let method_fragment = location.fragment().to_owned();

  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(did, setup.key_type, &public, method_fragment.name())?;

  let document = IotaDocument::from_verification_method(method)?;

  let mut state = IdentityState::new(document);

  // Store the generations at which the method was added
  state.store_method_generations(method_fragment);

  Ok(state)
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

        if state.document().resolve_method(method_url, None).is_some() {
          return Err(crate::Error::DIDError(identity_did::Error::MethodAlreadyExists));
        }

        let public: PublicKey = if let Some(method_private_key) = method_secret {
          insert_method_secret(storage, did, &location, type_, method_private_key).await
        } else {
          storage.key_new(did, &location).await.map_err(Into::into)
        }?;

        let key_type: KeyType = match type_ {
          MethodType::Ed25519VerificationKey2018 => KeyType::Ed25519,
          MethodType::X25519KeyAgreementKey2019 => KeyType::X25519,
        };

        let method: IotaVerificationMethod =
          IotaVerificationMethod::new(did.to_owned(), key_type, &public, location.fragment().name())?;

        state.store_method_generations(location.fragment().clone());

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

      store.key_insert(did, location, private_key).await.map_err(Into::into)
    }
    MethodSecret::X25519(private_key) => {
      ensure!(
        private_key.as_ref().len() == x25519::SECRET_KEY_LENGTH,
        UpdateError::InvalidMethodSecret(format!(
          "an ed25519 private key requires {} bytes, found {}",
          x25519::SECRET_KEY_LENGTH,
          private_key.as_ref().len()
        ))
      );

      ensure!(
        matches!(method_type, MethodType::X25519KeyAgreementKey2019),
        UpdateError::InvalidMethodSecret(
          "MethodType::X25519KeyAgreementKey2019 can only be used with an x25519 method secret".to_owned(),
        )
      );

      store.key_insert(did, location, private_key).await.map_err(Into::into)
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
