// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use did_doc::Document;
use did_url::DID;
use serde::{Deserialize, Serialize};

use crate::{
    error::Result,
    resolver::{DocumentMetadata, InputMetadata},
};

/// A resolved [`Document`] and associated [`DocumentMetadata`].
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MetaDocument {
    /// A resolved DID Document.
    pub data: Document,
    /// Information regarding the associated Documents resolution process.
    pub meta: DocumentMetadata,
}

/// A trait for generic DID Resolvers.
#[async_trait(?Send)]
pub trait ResolverMethod {
    /// Returns `true` if the given `did` is supported by this DID Resolver.
    fn is_supported(&self, did: &DID) -> bool;

    /// Performs the "Read" operation of the DID method.
    async fn read(&self, did: &DID, input: InputMetadata) -> Result<Option<MetaDocument>>;
}

#[async_trait(?Send)]
impl<T> ResolverMethod for &'_ T
where
    T: ResolverMethod + Send + Sync,
{
    fn is_supported(&self, did: &DID) -> bool {
        (**self).is_supported(did)
    }

    async fn read(&self, did: &DID, input: InputMetadata) -> Result<Option<MetaDocument>> {
        (**self).read(did, input).await
    }
}
