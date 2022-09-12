// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_account_storage::identity::ChainState;
use identity_account_storage::types::CekAlgorithm;
use identity_account_storage::types::EncryptedData;
use identity_account_storage::types::EncryptionAlgorithm;
use identity_account_storage::types::KeyLocation;
use identity_account_storage::types::Signature;
use identity_did::did::CoreDID;
use identity_iota_core_legacy::document::IotaDocument;
use napi::Result;
use napi_derive::napi;

use crate::error::NapiResult;

/// Creates a simple wrapper struct that will be exported to TypeScript
/// with helper functions for JSON conversion.
macro_rules! derive_napi_class {
  ($rust_struct:ident, $napi_class:ident) => {
    #[napi]
    pub struct $napi_class(pub(crate) $rust_struct);

    #[napi]
    impl $napi_class {
      #[napi(js_name = fromJSON)]
      pub fn from_json(json_value: serde_json::Value) -> Result<$napi_class> {
        serde_json::from_value(json_value).map(Self).napi_result()
      }

      #[napi(js_name = toJSON)]
      pub fn to_json(&self) -> Result<serde_json::Value> {
        serde_json::to_value(&self.0).napi_result()
      }
    }

    impl From<$rust_struct> for $napi_class {
      fn from(value: $rust_struct) -> Self {
        $napi_class(value)
      }
    }
  };
}

derive_napi_class!(CekAlgorithm, NapiCekAlgorithm);
derive_napi_class!(ChainState, NapiChainState);
derive_napi_class!(EncryptionAlgorithm, NapiEncryptionAlgorithm);
derive_napi_class!(EncryptedData, NapiEncryptedData);
derive_napi_class!(CoreDID, NapiCoreDid);
derive_napi_class!(IotaDocument, NapiDocument);
derive_napi_class!(KeyLocation, NapiKeyLocation);
derive_napi_class!(Signature, NapiSignature);
