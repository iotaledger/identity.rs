// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::future::join_all;
use futures::FutureExt;
use identity_core::convert::FromJson;

use crate::account::AccountBuilder;
use crate::error::Result;
use crate::identity::Identity;
use crate::identity::IdentityBuilder;
use crate::identity::IdentityHandle;
use crate::storage::StorageHandle;
use crate::utils::GenericCache;

#[derive(Debug)]
pub struct Account {
  storage: StorageHandle,
  cache_identities: GenericCache<IdentityHandle>,
  cache_credentials: GenericCache<()>,
  cache_presentations: GenericCache<()>,
}

impl Account {
  pub fn builder() -> AccountBuilder {
    AccountBuilder::new()
  }

  pub fn new(storage: StorageHandle) -> Self {
    Self {
      storage,
      cache_identities: GenericCache::new(),
      cache_credentials: GenericCache::new(),
      cache_presentations: GenericCache::new(),
    }
  }

  pub fn create_identity(&self) -> IdentityBuilder<'_> {
    IdentityBuilder::new(&self.storage, &self.cache_identities)
  }

  pub async fn identities(&self) -> Result<Vec<IdentityHandle>> {
    let guard: _ = self.cache_identities.read()?;

    let futures: _ = guard
      .values()
      .map(|identity| identity.index().map(move |index| (index, identity)));

    let mut output: Vec<(u32, &IdentityHandle)> = join_all(futures).await;

    output.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(output.into_iter().map(|(_, identity)| identity.clone()).collect())
  }

  pub(crate) async fn initialize(&self) -> Result<()> {
    // Load all identities into the cache
    let identities: Vec<Vec<u8>> = self.storage.all().await?;
    let mut cache: _ = self.cache_identities.write()?;

    for data in identities {
      let identity: Identity = Identity::from_json_slice(&data)?;
      let identity_id: String = identity.id.to_string();

      cache.insert(identity_id, IdentityHandle::new(identity));
    }

    Ok(())
  }
}
