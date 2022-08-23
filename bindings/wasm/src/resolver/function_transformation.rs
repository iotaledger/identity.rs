// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::ValidatorDocument;
use identity_iota::resolver::Error;
use identity_iota::resolver::Result;
use js_sys::Function;
use js_sys::Promise;
use std::future::Future;
use std::pin::Pin;
use wasm_bindgen::prelude::*;

type AsyncFnPtr<S, T> = Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r>>>;
use wasm_bindgen_futures::JsFuture;

use crate::error::ErrorString;
use crate::error::JsValueResult;

use super::supported_document_types::RustSupportedDocument;

pub(super) struct WasmResolverCommand {
  pub(super) ptr: AsyncFnPtr<str, Result<Box<dyn ValidatorDocument>>>,
}

impl WasmResolverCommand {
  pub(super) fn new(fun: &Function) -> Self {
    let fun_closure_clone = fun.clone();
    let ptr: AsyncFnPtr<str, Result<Box<dyn ValidatorDocument>>> = Box::new(move |input: &str| {
      let fun_clone = fun_closure_clone.clone();
      Box::pin(async move {
        let closure_output_promise: Promise = Promise::resolve(
          &JsValueResult::from(fun_clone.call1(&JsValue::null(), &input.into()))
            .stringify_error()
            .map_err(|error| Error::HandlerError(error.into()))?,
        );
        let awaited_output = JsValueResult::from(JsFuture::from(closure_output_promise).await)
          .stringify_error()
          .map_err(|error| Error::HandlerError(error.into()))?;

        let supported_document: RustSupportedDocument = awaited_output.into_serde().map_err(|error| {
          Error::HandlerError(
            ErrorString(format!(
              "resolution succeeded, but could not convert the outcome into a supported DID Document: {}",
              error.to_string()
            ))
            .into(),
          )
        })?;
        Ok(Box::<dyn ValidatorDocument>::from(supported_document))
      })
    });

    Self { ptr }
  }
}
