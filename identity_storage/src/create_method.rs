// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verification::MethodType;
use identity_did::verification::VerificationMethod;

use crate::BlobStorage;
use crate::KeyStorage;
use crate::MethodContent;
use crate::MethodSuite;
use crate::Storage;

pub struct CreateMethodBuilder<'builder, K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  document: &'builder mut CoreDocument,
  method_suite: Option<&'builder MethodSuite<K, B>>,
  content: Option<MethodContent>,
  typ: Option<MethodType>,
  fragment: Option<String>,
}

impl<'builder, K, B> CreateMethodBuilder<'builder, K, B>
where
  K: KeyStorage,
  B: BlobStorage,
{
  pub fn new(document: &'builder mut CoreDocument) -> Self {
    Self {
      document,
      method_suite: None,
      content: None,
      fragment: None,
      typ: None,
    }
  }

  pub fn content(mut self, content: MethodContent) -> Self {
    self.content = Some(content);
    self
  }

  pub fn type_(mut self, typ: MethodType) -> Self {
    self.typ = Some(typ);
    self
  }

  pub fn fragment(mut self, fragment: &str) -> Self {
    self.fragment = Some(fragment.to_owned());
    self
  }

  pub fn method_suite(mut self, method_suite: &'builder MethodSuite<K, B>) -> Self {
    self.method_suite = Some(method_suite);
    self
  }

  pub async fn apply(self) {
    let method_suite = self.method_suite.expect("TODO");
    let method_type = self.typ.expect("TODO");
    let method_content = self.content.expect("TODO");

    // TODO: Store key_alias mapping to method id.
    // TODO: Allow user or suite to also set method custom properties (?)
    let (key_alias, method_data) = method_suite.create(&method_type, method_content).await;

    // let identity_state: IdentityState = self.load_identity_state(did_url, storage).await?;
    // let mut method_index = identity_state.method_index().expect("TODO").unwrap_or_default();

    let method = VerificationMethod::builder(Default::default())
      .id(
        self
          .document
          .id()
          .to_owned()
          .join(self.fragment.expect("TODO"))
          .expect("TODO"),
      )
      .controller(self.document.id().to_owned())
      .data(method_data)
      .type_(method_type)
      .build()
      .expect("TODO");

    self.document.insert_method(method, Default::default()).expect("TODO");
  }

  // async fn load_identity_state<B: BlobStorage>(
  //   &self,
  //   did_url: &CoreDIDUrl,
  //   storage: &Storage<K, B>,
  // ) -> StorageResult<IdentityState> {
  //   let identity_state: IdentityState = match storage
  //     .load(did_url.did())
  //     .await?
  //     .and_then(|serialize_state| Some(IdentityState::from_json_slice(&serialize_state)))
  //   {
  //     Some(deserialization_result) => deserialization_result.expect("TODO"),
  //     None => IdentityState::new(None).expect("TODO"),
  //   };

  //   Ok(identity_state)
  // }
}
