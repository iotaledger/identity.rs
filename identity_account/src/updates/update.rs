// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::types::DIDType;
use identity_did::did::CoreDID;
use log::debug;
use log::trace;

use identity_account_storage::storage::Storage;
use identity_account_storage::types::KeyLocation;
use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use identity_did::did::DID;
use identity_did::service::Service;
use identity_did::service::ServiceEndpoint;
use identity_did::utils::Queryable;
use identity_did::verification::MethodRef;
use identity_did::verification::MethodRelationship;
use identity_did::verification::MethodScope;
use identity_iota_client::tangle::Client;
use identity_iota_client::tangle::SharedPtr;
use identity_iota_core_legacy::did::IotaDID;
use identity_iota_core_legacy::did::IotaDIDUrl;
use identity_iota_core_legacy::document::IotaDocument;
use identity_iota_core_legacy::document::IotaService;
use identity_iota_core_legacy::document::IotaVerificationMethod;
use identity_iota_core_legacy::tangle::NetworkName;

use crate::account::Account;
use crate::error::Result;
use crate::types::IdentitySetup;
use crate::types::MethodContent;
use crate::updates::UpdateError;

pub(crate) async fn create_identity(
  setup: IdentitySetup,
  network: NetworkName,
  store: &dyn Storage,
) -> Result<IotaDocument> {
  let fragment: &str = IotaDocument::DEFAULT_METHOD_FRAGMENT;

  if let Some(private_key) = &setup.private_key {
    KeyPair::try_from_private_key_bytes(KeyType::Ed25519, private_key.as_ref())
      .map_err(|err| UpdateError::InvalidMethodContent(err.to_string()))?;
  };

  let (did, location) = store
    .did_create(DIDType::IotaDID, network.clone(), fragment, setup.private_key)
    .await?;

  let public_key: PublicKey = store.key_public(&did, &location).await?;

  let method: IotaVerificationMethod =
    IotaVerificationMethod::new(did.clone().try_into()?, KeyType::Ed25519, &public_key, fragment)?;

  let document = IotaDocument::from_verification_method(method)?;

  Ok(document)
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
    types: Vec<String>,
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
  pub(crate) async fn process(self, did: &IotaDID, document: &mut IotaDocument, storage: &dyn Storage) -> Result<()> {
    debug!("[Update::process] Update = {:?}", self);
    trace!("[Update::process] Document = {:?}", document);
    trace!("[Update::process] Store = {:?}", storage);

    match self {
      Self::CreateMethod {
        scope,
        content,
        fragment,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        // Check method identifier is not duplicated.
        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;
        if document.resolve_method(method_url, None).is_some() {
          return Err(crate::Error::DIDError(identity_did::Error::MethodAlreadyExists));
        }

        // Generate or extract the private key and/or retrieve the public key.
        let key_type: KeyType = content.key_type();
        let public: PublicKey = match content {
          MethodContent::GenerateEd25519 | MethodContent::GenerateX25519 => {
            let location: KeyLocation = storage.key_generate(did.as_ref(), key_type, fragment.name()).await?;
            storage.key_public(did.as_ref(), &location).await?
          }
          MethodContent::PrivateEd25519(private_key) | MethodContent::PrivateX25519(private_key) => {
            let location: KeyLocation =
              insert_method_secret(storage, did.as_ref(), key_type, fragment.name(), private_key).await?;
            storage.key_public(did.as_ref(), &location).await?
          }
          MethodContent::PublicEd25519(public_key) => public_key,
          MethodContent::PublicX25519(public_key) => public_key,
        };

        // Insert a new method.
        let method: IotaVerificationMethod =
          IotaVerificationMethod::new(did.clone(), key_type, &public, fragment.name())?;

        document.insert_method(method, scope)?;
      }
      Self::DeleteMethod { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // Prevent deleting the last method capable of signing the DID document.
        let capability_invocation_set = document.core_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        document.remove_method(&method_url)?;
      }
      Self::AttachMethodRelationship {
        fragment,
        relationships,
      } => {
        let fragment: Fragment = Fragment::new(fragment);

        let method_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        for relationship in relationships {
          // Ignore result: attaching is idempotent.
          let _ = document.attach_method_relationship(&method_url, relationship)?;
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
          document.core_document().capability_invocation();
        let is_capability_invocation = capability_invocation_set
          .iter()
          .any(|method_ref| method_ref.id() == &method_url);

        ensure!(
          !(is_capability_invocation && capability_invocation_set.len() == 1),
          UpdateError::InvalidMethodFragment("cannot remove last signing method")
        );

        for relationship in relationships {
          // Ignore result: detaching is idempotent.
          let _ = document.detach_method_relationship(&method_url, relationship)?;
        }
      }
      Self::CreateService {
        fragment,
        types,
        endpoint,
        properties,
      } => {
        let fragment = Fragment::new(fragment);
        let did_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // The service must not exist.
        ensure!(
          document.service().query(&did_url).is_none(),
          UpdateError::DuplicateServiceFragment(fragment.name().to_owned()),
        );

        let service: IotaService = Service::builder(properties.unwrap_or_default())
          .id(did_url)
          .service_endpoint(endpoint)
          .types(types)
          .build()?;

        document.insert_service(service);
      }
      Self::DeleteService { fragment } => {
        let fragment: Fragment = Fragment::new(fragment);
        let service_url: IotaDIDUrl = did.to_url().join(fragment.identifier())?;

        // The service must exist
        ensure!(
          document.service().query(&service_url).is_some(),
          UpdateError::ServiceNotFound
        );

        document.remove_service(&service_url);
      }
      Self::SetController { controllers } => {
        *document.controller_mut() = controllers;
      }

      Self::SetAlsoKnownAs { urls } => {
        *document.also_known_as_mut() = urls;
      }
    }

    document.metadata.updated = Some(Timestamp::now_utc());

    Ok(())
  }
}

async fn insert_method_secret(
  store: &dyn Storage,
  did: &CoreDID,
  key_type: KeyType,
  fragment: &str,
  private_key: PrivateKey,
) -> Result<KeyLocation> {
  let keypair: KeyPair = KeyPair::try_from_private_key_bytes(key_type, private_key.as_ref())
    .map_err(|err| UpdateError::InvalidMethodContent(err.to_string()))?;

  let location: KeyLocation = KeyLocation::new(key_type, fragment.to_owned(), keypair.public().as_ref());
  std::mem::drop(keypair);

  ensure!(
    !store.key_exists(did, &location).await?,
    UpdateError::DuplicateKeyLocation(location)
  );

  store.key_insert(did, &location, private_key).await?;

  Ok(location)
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
/// - `types`: the type/s of the service, e.g. `"LinkedDomains"`, required.
/// - `fragment`: the identifier of the service in the document, required.
/// - `endpoint`: the `ServiceEndpoint` of the service, required.
/// - `properties`: additional properties of the service, optional.
CreateService {
  @required fragment String,
  @required types Vec<String>,
  @required endpoint ServiceEndpoint,
  @optional properties Object,
});

impl<'account, C> CreateServiceBuilder<'account, C>
where
  C: SharedPtr<Client>,
{
  #[must_use]
  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.types.get_or_insert_with(Default::default).push(value.into());
    self
  }
}

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
