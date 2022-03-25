// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::x25519;
use crypto::signatures::ed25519;
use identity_account_storage::storage::Storage;
use identity_account_storage::types::AccountId;
use identity_account_storage::types::KeyLocation;
use identity_core::common::Fragment;
use identity_core::common::Object;
use identity_core::common::OneOrSet;
use identity_core::common::OrderedSet;
use identity_core::common::Timestamp;
use identity_core::common::Url;
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
use log::debug;
use log::trace;

use crate::account::Account;
use crate::error::Result;
use crate::identity::IdentitySetup;
use crate::types::MethodSecret;
use crate::updates::UpdateError;

pub(crate) async fn create_identity(
  setup: IdentitySetup,
  network: NetworkName,
  store: &dyn Storage,
) -> Result<(AccountId, IotaDocument)> {
  let method_type = match setup.key_type {
    KeyType::Ed25519 => MethodType::Ed25519VerificationKey2018,
    KeyType::X25519 => MethodType::X25519KeyAgreementKey2019,
  };

  // The method type must be able to sign document updates.
  ensure!(
    IotaDocument::is_signing_method_type(method_type),
    UpdateError::InvalidMethodType(method_type)
  );

  let account_id: AccountId = AccountId::generate();

  let tmp_location: KeyLocation = KeyLocation::random(setup.key_type);

  if let Some(inner_method_secret @ MethodSecret::Ed25519(_)) = setup.method_secret {
    insert_method_secret(store, account_id, &tmp_location, setup.key_type, inner_method_secret).await?;
  } else {
    store.key_generate(&account_id, &tmp_location).await?;
  }

  let public_key: PublicKey = store.key_public(&account_id, &tmp_location).await?;

  // Generate a new DID from the public key
  let did: IotaDID = IotaDID::new_with_network(public_key.as_ref(), network)?;

  ensure!(
    store.index_get(&did).await?.is_none(),
    UpdateError::DocumentAlreadyExists
  );

  let method: IotaVerificationMethod = IotaVerificationMethod::new(
    did.clone(),
    setup.key_type,
    &public_key,
    IotaDocument::DEFAULT_METHOD_FRAGMENT,
  )?;
  let location: KeyLocation = KeyLocation::from_verification_method(&method)?;

  // The key location must be available.
  if store.key_exists(&account_id, &location).await? {
    store.key_del(&account_id, &tmp_location).await?;
    return Err(UpdateError::DuplicateKeyLocation(location).into());
  }

  store.key_move(&account_id, &tmp_location, &location).await?;

  store.index_set(did, account_id).await?;

  let document = IotaDocument::from_verification_method(method)?;

  Ok((account_id, document))
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
  pub(crate) async fn process(
    self,
    did: &IotaDID,
    account_id: AccountId,
    document: &mut IotaDocument,
    storage: &dyn Storage,
  ) -> Result<()> {
    debug!("[Update::process] Update = {:?}", self);
    trace!("[Update::process] State = {:?}", document);
    trace!("[Update::process] Store = {:?}", storage);

    match self {
      Self::CreateMethod {
        type_,
        scope,
        fragment,
        method_secret,
      } => {
        let key_type: KeyType = method_to_key_type(type_);

        // TODO: Done to ensure a leading `#`. Should be replaced eventually.
        let fragment: Fragment = Fragment::new(fragment);
        let method_url: CoreDIDUrl = did.as_ref().to_url().join(fragment.identifier())?;

        if document.resolve_method(method_url).is_some() {
          return Err(crate::Error::DIDError(identity_did::Error::MethodAlreadyExists));
        }

        let tmp_location: KeyLocation = KeyLocation::random(key_type);

        if let Some(method_private_key) = method_secret {
          insert_method_secret(storage, account_id, &tmp_location, key_type, method_private_key).await?;
        } else {
          storage.key_generate(&account_id, &tmp_location).await?;
        };

        let public_key: PublicKey = storage.key_public(&account_id, &tmp_location).await?;

        let method: IotaVerificationMethod =
          IotaVerificationMethod::new(did.clone(), KeyType::Ed25519, &public_key, fragment.name())?;

        let location: KeyLocation = KeyLocation::from_verification_method(&method)?;

        // The key location must be available.
        if storage.key_exists(&account_id, &location).await? {
          storage.key_del(&account_id, &tmp_location).await?;
          return Err(UpdateError::DuplicateKeyLocation(location).into());
        }

        // Move the key from the tmp to the expected location.
        storage.key_move(&account_id, &tmp_location, &location).await?;

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
        type_,
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
          .type_(type_)
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

        document.remove_service(&service_url)?;
      }
      Self::SetController { controllers } => {
        *document.controller_mut() = controllers;
      }

      Self::SetAlsoKnownAs { urls } => {
        *document.also_known_as_mut() = urls;
      }
    }

    document.metadata.updated = Timestamp::now_utc();

    Ok(())
  }
}

async fn insert_method_secret(
  store: &dyn Storage,
  account_id: AccountId,
  location: &KeyLocation,
  key_type: KeyType,
  method_secret: MethodSecret,
) -> Result<()> {
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
        matches!(key_type, KeyType::Ed25519),
        UpdateError::InvalidMethodSecret("KeyType::Ed25519 can only be used with an ed25519 method secret".to_owned(),)
      );

      store
        .key_insert(&account_id, location, private_key)
        .await
        .map_err(Into::into)
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
        matches!(key_type, KeyType::X25519),
        UpdateError::InvalidMethodSecret(
          "MethodType::X25519KeyAgreementKey2019 can only be used with an x25519 method secret".to_owned(),
        )
      );

      store
        .key_insert(&account_id, location, private_key)
        .await
        .map_err(Into::into)
    }
    MethodSecret::MerkleKeyCollection(_) => {
      todo!("[Update::CreateMethod] Handle MerkleKeyCollection")
    }
  }
}

pub(crate) fn method_to_key_type(method_type: MethodType) -> KeyType {
  match method_type {
    MethodType::Ed25519VerificationKey2018 => KeyType::Ed25519,
    MethodType::X25519KeyAgreementKey2019 => KeyType::X25519,
    MethodType::MerkleKeyCollection2021 => todo!(),
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
