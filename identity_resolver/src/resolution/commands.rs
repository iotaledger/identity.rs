// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use core::future::Future;
use std::pin::Pin;

pub trait Command<'a, T>: private::Sealed {
  type Output: Future<Output = T> + 'a;
  fn apply(&self, input: &'a str) -> Self::Output;
}

//type AsyncFnPtr<S,T> = Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r>>>;
type SendSyncAsyncFnPtr<S, T> =
  Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r + Send + Sync>> + Send + Sync>;
pub(super) type SendSyncCommand<DOC> = SendSyncAsyncFnPtr<str, Result<DOC>>;

impl<'a, DOC: Send + Sync + 'static> Command<'a, Result<DOC>> for SendSyncCommand<DOC> {
  type Output = Pin<Box<dyn Future<Output = Result<DOC>> + 'a + Send + Sync>>;
  fn apply(&self, input: &'a str) -> Self::Output {
    self(input)
  }
}

mod private {
  use super::SendSyncCommand;
  pub trait Sealed {}
  impl<DOC: Send + Sync + 'static> Sealed for SendSyncCommand<DOC> {}
}
