// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Context;
use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::common::Value;

use crate::credential::Policy;
use crate::credential::Refresh;
use crate::credential::VerifiableCredential;
use crate::error::Result;
use crate::presentation::Presentation;

/// A `PresentationBuilder` is used to create a customized `Presentation`.
#[derive(Clone, Debug)]
pub struct PresentationBuilder<T = Object, U = Object> {
  pub(crate) context: Vec<Context>,
  pub(crate) id: Option<Url>,
  pub(crate) types: Vec<String>,
  pub(crate) credentials: Vec<VerifiableCredential<U>>,
  pub(crate) holder: Option<Url>,
  pub(crate) refresh: Vec<Refresh>,
  pub(crate) policy: Vec<Policy>,
  pub(crate) properties: T,
}

impl<T, U> PresentationBuilder<T, U> {
  /// Creates a new `PresentationBuilder`.
  pub fn new(properties: T) -> Self {
    Self {
      context: vec![Presentation::<T, U>::base_context().clone()],
      id: None,
      types: vec![Presentation::<T, U>::base_type().into()],
      credentials: Vec::new(),
      holder: None,
      refresh: Vec::new(),
      policy: Vec::new(),
      properties,
    }
  }

  /// Adds a value to the `Presentation` context set.
  #[must_use]
  pub fn context(mut self, value: impl Into<Context>) -> Self {
    self.context.push(value.into());
    self
  }

  /// Sets the value of the `Presentation` `id`.
  #[must_use]
  pub fn id(mut self, value: Url) -> Self {
    self.id = Some(value);
    self
  }

  /// Adds a value to the `Presentation` type set.
  #[must_use]
  pub fn type_(mut self, value: impl Into<String>) -> Self {
    self.types.push(value.into());
    self
  }

  /// Adds a value to the `verifiableCredential` set.
  #[must_use]
  pub fn credential(mut self, value: VerifiableCredential<U>) -> Self {
    self.credentials.push(value);
    self
  }

  /// Sets the value of the `Credential` `holder`.
  #[must_use]
  pub fn holder(mut self, value: Url) -> Self {
    self.holder = Some(value);
    self
  }

  /// Adds a value to the `refreshService` set.
  #[must_use]
  pub fn refresh(mut self, value: Refresh) -> Self {
    self.refresh.push(value);
    self
  }

  /// Adds a value to the `termsOfUse` set.
  #[must_use]
  pub fn policy(mut self, value: Policy) -> Self {
    self.policy.push(value);
    self
  }

  /// Returns a new `Presentation` based on the `PresentationBuilder` configuration.
  pub fn build(self) -> Result<Presentation<T, U>> {
    Presentation::from_builder(self)
  }
}

impl<T> PresentationBuilder<Object, T> {
  /// Adds a new custom property to the `Presentation`.
  #[must_use]
  pub fn property<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<Value>,
  {
    self.properties.insert(key.into(), value.into());
    self
  }

  /// Adds a series of custom properties to the `Presentation`.
  #[must_use]
  pub fn properties<K, V, I>(mut self, iter: I) -> Self
  where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<Value>,
  {
    self
      .properties
      .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    self
  }
}

impl<T, U> Default for PresentationBuilder<T, U>
where
  T: Default,
{
  fn default() -> Self {
    Self::new(T::default())
  }
}

#[cfg(test)]
mod tests {
  use identity_core::common::Object;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::utils::encode_b58;
  use identity_did::did::DID;
  use identity_did::document::Document;
  use identity_did::document::DocumentBuilder;
  use identity_did::verification::Method;
  use identity_did::verification::MethodBuilder;
  use identity_did::verification::MethodData;
  use identity_did::verification::MethodType;
  use serde_json::json;
  use serde_json::Value;

  use crate::credential::Credential;
  use crate::credential::CredentialBuilder;
  use crate::credential::Subject;
  use crate::credential::VerifiableCredential;
  use crate::presentation::Presentation;
  use crate::presentation::PresentationBuilder;

  fn subject() -> Subject {
    let json: Value = json!({
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    });

    Subject::from_json_value(json).unwrap()
  }

  fn issuer() -> Url {
    Url::parse("did:example:issuer").unwrap()
  }

  #[test]
  fn test_presentation_builder_valid() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    let controller: DID = "did:example:1234".parse().unwrap();

    let method: Method = MethodBuilder::default()
      .id(controller.join("#key-1").unwrap())
      .controller(controller.clone())
      .key_type(MethodType::Ed25519VerificationKey2018)
      .key_data(MethodData::PublicKeyBase58(encode_b58(keypair.public())))
      .build()
      .unwrap();

    let document: Document = DocumentBuilder::default()
      .id(controller)
      .verification_method(method)
      .build()
      .unwrap();

    let credential: VerifiableCredential = CredentialBuilder::default()
      .type_("ExampleCredential")
      .subject(subject())
      .issuer(issuer())
      .build()
      .unwrap()
      .sign(&document, "#key-1".into(), keypair.secret())
      .unwrap();

    let presentation: Presentation = PresentationBuilder::default()
      .type_("ExamplePresentation")
      .credential(credential)
      .build()
      .unwrap();

    assert_eq!(presentation.context.len(), 1);
    assert_eq!(
      presentation.context.get(0).unwrap(),
      Presentation::<Object>::base_context()
    );
    assert_eq!(presentation.types.len(), 2);
    assert_eq!(presentation.types.get(0).unwrap(), Presentation::<Object>::base_type());
    assert_eq!(presentation.types.get(1).unwrap(), "ExamplePresentation");
    assert_eq!(presentation.verifiable_credential.len(), 1);
    assert_eq!(
      presentation.verifiable_credential.get(0).unwrap().types.get(0).unwrap(),
      Credential::<Object>::base_type()
    );
    assert_eq!(
      presentation.verifiable_credential.get(0).unwrap().types.get(1).unwrap(),
      "ExampleCredential"
    );
  }
}
