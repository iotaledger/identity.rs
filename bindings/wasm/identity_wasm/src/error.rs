// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::CompoundJwtPresentationValidationError;
use identity_iota::resolver;
use identity_iota::storage::key_id_storage::KeyIdStorageError;
use identity_iota::storage::key_id_storage::KeyIdStorageErrorKind;
use identity_iota::storage::key_id_storage::KeyIdStorageResult;
use identity_iota::storage::key_storage::KeyStorageError;
use identity_iota::storage::key_storage::KeyStorageErrorKind;
use identity_iota::storage::key_storage::KeyStorageResult;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::result::Result as StdResult;
use tokio::sync::TryLockError;
use wasm_bindgen::JsValue;

impl_wasm_error_from!(
  identity_iota::core::Error,
  identity_iota::credential::Error,
  identity_iota::did::Error,
  identity_iota::document::Error,
  identity_iota::iota::Error,
  identity_iota::credential::JwtValidationError,
  identity_iota::credential::RevocationError,
  identity_iota::verification::Error,
  identity_iota::credential::DomainLinkageValidationError,
  identity_iota::sd_jwt_payload::Error,
  identity_iota::credential::KeyBindingJwtError,
  identity_iota::credential::status_list_2021::StatusListError,
  identity_iota::credential::status_list_2021::StatusList2021CredentialError
  identity_iota::iota::rebased::Error
);

impl_wasm_error_from_with_struct_name!(jsonprooftoken::errors::CustomError);

impl From<resolver::Error> for WasmError<'_> {
  fn from(error: resolver::Error) -> Self {
    Self {
      name: Cow::Owned(format!("ResolverError::{}", <&'static str>::from(error.error_cause()))),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::iota::block::Error> for WasmError<'_> {
  fn from(error: identity_iota::iota::block::Error) -> Self {
    Self {
      name: Cow::Borrowed("iota_sdk::types::block::Error"),
      message: Cow::Owned(error.to_string()),
    }
  }
}

impl From<identity_iota::credential::CompoundCredentialValidationError> for WasmError<'_> {
  fn from(error: identity_iota::credential::CompoundCredentialValidationError) -> Self {
    Self {
      name: Cow::Borrowed("CompoundCredentialValidationError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::core::SingleStructError<KeyStorageErrorKind>> for WasmError<'_> {
  fn from(error: identity_iota::core::SingleStructError<KeyStorageErrorKind>) -> Self {
    Self {
      name: Cow::Borrowed("KeyStorageError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::core::SingleStructError<KeyIdStorageErrorKind>> for WasmError<'_> {
  fn from(error: identity_iota::core::SingleStructError<KeyIdStorageErrorKind>) -> Self {
    Self {
      name: Cow::Borrowed("KeyIdStorageError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::storage::key_id_storage::MethodDigestConstructionError> for WasmError<'_> {
  fn from(error: identity_iota::storage::key_id_storage::MethodDigestConstructionError) -> Self {
    Self {
      name: Cow::Borrowed("MethodDigestConstructionError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::storage::storage::JwkStorageDocumentError> for WasmError<'_> {
  fn from(error: identity_iota::storage::storage::JwkStorageDocumentError) -> Self {
    Self {
      name: Cow::Borrowed("JwkDocumentExtensionError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::verification::jws::SignatureVerificationError> for WasmError<'_> {
  fn from(error: identity_iota::verification::jws::SignatureVerificationError) -> Self {
    Self {
      name: Cow::Borrowed("SignatureVerificationError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<identity_iota::verification::jose::error::Error> for WasmError<'_> {
  fn from(error: identity_iota::verification::jose::error::Error) -> Self {
    Self {
      name: Cow::Borrowed("JoseError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<CompoundJwtPresentationValidationError> for WasmError<'_> {
  fn from(error: CompoundJwtPresentationValidationError) -> Self {
    Self {
      name: Cow::Borrowed("CompoundJwtPresentationValidationError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<TryLockError> for WasmError<'_> {
  fn from(error: TryLockError) -> Self {
    Self {
      name: Cow::Borrowed("TryLockError"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

impl From<secret_storage::Error> for WasmError<'_> {
  fn from(error: secret_storage::Error) -> Self {
    Self {
      name: Cow::Borrowed("secret_storage::Error"),
      message: Cow::Owned(ErrorMessage(&error).to_string()),
    }
  }
}

/// Convenience struct to convert Result<JsValue, JsValue> to errors in the Rust library.
pub struct JsValueResult(pub(crate) Result<JsValue>);

impl JsValueResult {
  /// Consumes the struct and returns a Result<_, KeyStorageError>, leaving an `Ok` value untouched.
  pub fn to_key_storage_error(self) -> KeyStorageResult<JsValue> {
    self
      .stringify_error()
      .map_err(|err| KeyStorageError::new(KeyStorageErrorKind::Unspecified).with_source(err))
  }

  pub fn to_key_id_storage_error(self) -> KeyIdStorageResult<JsValue> {
    self
      .stringify_error()
      .map_err(|err| KeyIdStorageError::new(KeyIdStorageErrorKind::Unspecified).with_source(err))
  }

  // Consumes the struct and returns a Result<_, String>, leaving an `Ok` value untouched.
  pub(crate) fn stringify_error(self) -> StdResult<JsValue, String> {
    stringify_js_error(self.0)
  }

  /// Consumes the struct and returns a Result<_, identity_iota::iota::Error>, leaving an `Ok` value untouched.
  pub fn to_iota_core_error(self) -> StdResult<JsValue, identity_iota::iota::Error> {
    self.stringify_error().map_err(identity_iota::iota::Error::JsError)
  }

  pub fn to_kinesis_client_error(self) -> StdResult<JsValue, identity_iota::iota::sui_name_tbd_error::Error> {
    self
      .stringify_error()
      .map_err(|e| identity_iota::iota::sui_name_tbd_error::Error::FfiError(e.to_string()))
  }
}

impl From<Result<JsValue>> for JsValueResult {
  fn from(result: Result<JsValue>) -> Self {
    JsValueResult(result)
  }
}

impl<T: for<'a> serde::Deserialize<'a>> From<JsValueResult> for KeyStorageResult<T> {
  fn from(result: JsValueResult) -> Self {
    result.to_key_storage_error().and_then(|js_value| {
      js_value
        .into_serde()
        .map_err(|e| KeyStorageError::new(KeyStorageErrorKind::SerializationError).with_source(e))
    })
  }
}

impl<T: for<'a> serde::Deserialize<'a>> From<JsValueResult> for KeyIdStorageResult<T> {
  fn from(result: JsValueResult) -> Self {
    result.to_key_id_storage_error().and_then(|js_value| {
      js_value
        .into_serde()
        .map_err(|e| KeyIdStorageError::new(KeyIdStorageErrorKind::SerializationError).with_source(e))
    })
  }
}

impl<T: for<'a> serde::Deserialize<'a>> From<JsValueResult> for StdResult<T, identity_iota::iota::sui_name_tbd_error::Error> {
  fn from(result: JsValueResult) -> Self {
    result.to_kinesis_client_error().and_then(|js_value| {
      js_value
        .into_serde()
        .map_err(|e| identity_iota::iota::sui_name_tbd_error::Error::FfiError(e.to_string()))
    })
  }
}
