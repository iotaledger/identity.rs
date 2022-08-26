// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::future::Future;
use identity_credential::validator::BorrowValidator;
use identity_did::did::DID;

use crate::Error;
use crate::Result;
use std::pin::Pin;

pub trait Command<'a, T>: private::Sealed {
  type Output: Future<Output = T> + 'a;
  fn apply(&self, input: &'a str) -> Self::Output;
}


pub(super) type AsyncFnPtr<S, T> = Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r>>>;

type SendSyncAsyncFnPtr<S, T> =
  Box<dyn for<'r> Fn(&'r S) -> Pin<Box<dyn Future<Output = T> + 'r + Send + Sync>> + Send + Sync>;

pub struct SendSyncCommand<DOC: BorrowValidator + Send + Sync + 'static> {
  fun: SendSyncAsyncFnPtr<str, Result<DOC>>,
}

impl<'a, DOC: BorrowValidator + Send + Sync + 'static> Command<'a, Result<DOC>> for SendSyncCommand<DOC> {
  type Output = Pin<Box<dyn Future<Output = Result<DOC>> + 'a + Send + Sync>>;
  fn apply(&self, input: &'a str) -> Self::Output {
    (self.fun)(input)
  }
}

pub struct SingleThreadedCommand<DOC> {
  pub(super) fun: AsyncFnPtr<str, Result<DOC>>,
}
impl<'a, DOC: BorrowValidator + 'static> Command<'a, Result<DOC>> for SingleThreadedCommand<DOC> {
  type Output = Pin<Box<dyn Future<Output = Result<DOC>> + 'a>>;
  fn apply(&self, input: &'a str) -> Self::Output {
    (self.fun)(input)
  }
}

impl<DOC: BorrowValidator + Send + Sync + 'static> SendSyncCommand<DOC> {
  pub(super) fn new<D, F, Fut, DOCUMENT, E, DIDERR>(handler: F) -> Self
  where
    D: DID + Send + for<'r> TryFrom<&'r str, Error = DIDERR> + Sync + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone + Send + Sync,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>> + Send + Sync,
    E: std::error::Error + Send + Sync + 'static,
    DIDERR: std::error::Error + Send + Sync + 'static,
  {
    let fun: SendSyncAsyncFnPtr<str, Result<DOC>> = Box::new(move |input: &str| {
      let handler_clone = handler.clone();
      let did_parse_attempt = D::try_from(input).map_err(|error| Error::DIDParsingError {
        source: error.into(),
        context: crate::error::ResolutionAction::Unknown,
      });
      Box::pin(async move {
        let did: D = did_parse_attempt?;
        handler_clone(did)
          .await
          .map(Into::into)
          .map_err(|error| Error::HandlerError {
            source: error.into(),
            context: crate::error::ResolutionAction::Unknown,
          })
      })
    });
    Self { fun }
  }
}

impl<DOC: BorrowValidator + 'static> SingleThreadedCommand<DOC> {
  pub(super) fn new<D, F, Fut, DOCUMENT, E, DIDERR>(handler: F) -> Self
  where
    D: DID + for<'r> TryFrom<&'r str, Error = DIDERR> + 'static,
    DOCUMENT: 'static + Into<DOC>,
    F: Fn(D) -> Fut + 'static + Clone,
    Fut: Future<Output = std::result::Result<DOCUMENT, E>>,
    E: std::error::Error + Send + Sync + 'static,
    DIDERR: std::error::Error + Send + Sync + 'static,
  {
    let fun: AsyncFnPtr<str, Result<DOC>> = Box::new(move |input: &str| {
      let handler_clone = handler.clone();
      let did_parse_attempt = D::try_from(input).map_err(|error| Error::DIDParsingError {
        source: error.into(),
        context: crate::error::ResolutionAction::Unknown,
      });
      Box::pin(async move {
        let did: D = did_parse_attempt?;
        handler_clone(did)
          .await
          .map(Into::into)
          .map_err(|error| Error::HandlerError {
            source: error.into(),
            context: crate::error::ResolutionAction::Unknown,
          })
      })
    });
    Self { fun }
  }
}

mod private {
  use super::SendSyncCommand;
  use super::SingleThreadedCommand;
  use identity_credential::validator::BorrowValidator;
  pub trait Sealed {}
  impl<DOC: BorrowValidator + Send + Sync + 'static> Sealed for SendSyncCommand<DOC> {}
  impl<DOC: BorrowValidator + 'static> Sealed for SingleThreadedCommand<DOC> {}
}
