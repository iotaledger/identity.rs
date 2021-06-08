// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::comm::Uuid;
use identity::core::Url;
use identity::iota::IotaDID;

use crate::wasm_did::WasmDID;
use crate::wasm_url::WasmUrl;
use crate::wasm_uuid::WasmUuid;

pub trait IntoWasm {
  type Output;
  fn into_wasm(&self) -> Self::Output;
}

pub trait IntoRust: Sized {
  type Output;
  fn into_rust(&self) -> Self::Output;
}

// =============================================================================
// =============================================================================

impl IntoWasm for String {
  type Output = String;

  fn into_wasm(&self) -> Self::Output {
    self.clone()
  }
}

impl IntoRust for String {
  type Output = String;

  fn into_rust(&self) -> Self::Output {
    self.clone()
  }
}

// =============================================================================
// =============================================================================

impl IntoWasm for bool {
  type Output = bool;

  fn into_wasm(&self) -> Self::Output {
    *self
  }
}

impl IntoRust for bool {
  type Output = bool;

  fn into_rust(&self) -> Self::Output {
    *self
  }
}

// =============================================================================
// =============================================================================

impl IntoWasm for Uuid {
  type Output = WasmUuid;

  fn into_wasm(&self) -> Self::Output {
    self.clone().into()
  }
}

impl IntoRust for WasmUuid {
  type Output = Uuid;

  fn into_rust(&self) -> Self::Output {
    self.0.clone()
  }
}

// =============================================================================
// =============================================================================

impl IntoWasm for Url {
  type Output = WasmUrl;

  fn into_wasm(&self) -> Self::Output {
    self.clone().into()
  }
}

impl IntoRust for WasmUrl {
  type Output = Url;

  fn into_rust(&self) -> Self::Output {
    self.0.clone()
  }
}

// =============================================================================
// =============================================================================

impl IntoWasm for IotaDID {
  type Output = WasmDID;

  fn into_wasm(&self) -> Self::Output {
    self.clone().into()
  }
}

impl IntoRust for WasmDID {
  type Output = IotaDID;

  fn into_rust(&self) -> Self::Output {
    self.0.clone()
  }
}

// =============================================================================
// =============================================================================

impl<T: IntoWasm> IntoWasm for Option<T> {
  type Output = Option<T::Output>;

  fn into_wasm(&self) -> Self::Output {
    self.as_ref().map(T::into_wasm)
  }
}

impl<T: IntoRust> IntoRust for Option<T> {
  type Output = Option<T::Output>;

  fn into_rust(&self) -> Self::Output {
    self.as_ref().map(T::into_rust)
  }
}

// =============================================================================
// =============================================================================

impl<T: IntoWasm> IntoWasm for Vec<T> {
  type Output = Vec<T::Output>;

  fn into_wasm(&self) -> Self::Output {
    self.iter().map(T::into_wasm).collect()
  }
}

impl<T: IntoRust> IntoRust for Vec<T> {
  type Output = Vec<T::Output>;

  fn into_rust(&self) -> Self::Output {
    self.iter().map(T::into_rust).collect()
  }
}
