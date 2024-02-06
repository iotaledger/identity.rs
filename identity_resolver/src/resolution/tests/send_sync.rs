// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

use identity_did::DID;
use identity_document::document::CoreDocument;

fn is_send<T: Send>(_t: T) {}
fn is_send_sync<T: Send + Sync>(_t: T) {}

#[allow(dead_code)]
fn default_resolver_is_send_sync<DOC: AsRef<CoreDocument> + Send + Sync + 'static>() {
  let resolver = Resolver::<DOC>::new();
  is_send_sync(resolver);
}

#[allow(dead_code)]
fn resolver_methods_give_send_futures<DOC, D>(did: D)
where
  DOC: AsRef<CoreDocument> + Send + Sync + 'static,
  D: DID + Send + Sync + 'static,
{
  let resolver = Resolver::<DOC>::new();
  is_send(resolver.resolve(&did));
}
