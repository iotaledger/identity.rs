// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

use identity_credential::credential::Credential;
use identity_credential::presentation::Presentation;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::ValidatorDocument;
use identity_did::DID;
use serde::Serialize;

fn is_send<T: Send>(_t: T) {}
fn is_send_sync<T: Send + Sync>(_t: T) {}

#[allow(dead_code)]
fn default_resolver_is_send_sync<DOC: ValidatorDocument + Send + Sync + 'static>() {
  let resolver = Resolver::<DOC>::new();
  is_send_sync(resolver);
}

#[allow(dead_code)]
fn resolver_methods_give_send_futures<DOC, D, T, U, V>(
  did: D,
  credential: Credential<T>,
  presentation: Presentation<U, V>,
) where
  DOC: ValidatorDocument + Send + Sync + 'static,
  D: DID + Send + Sync + 'static,
  T: Send + Sync + Serialize,
  U: Send + Sync + Serialize,
  V: Send + Sync + Serialize,
{
  let resolver = Resolver::<DOC>::new();
  is_send(resolver.resolve(&did));

  is_send(resolver.resolve_credential_issuer(&credential));

  is_send(resolver.resolve_presentation_holder(&presentation));

  is_send(resolver.resolve_presentation_issuers(&presentation));

  is_send(resolver.verify_presentation(
    &presentation,
    &PresentationValidationOptions::default(),
    FailFast::FirstError,
    Option::<&DOC>::None,
    Option::<&[DOC]>::None,
  ));
}
