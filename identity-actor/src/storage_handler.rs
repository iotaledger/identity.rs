// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert;
use futures::stream::FuturesOrdered;
use futures::TryStreamExt;
use identity_account::account::Account;

use crate::types::{ActorRequest, StorageRequest, StorageResponse};
use crate::RequestHandler;

pub struct IdentityStorageHandler {
  account: Account,
}

impl IdentityStorageHandler {
  pub async fn new() -> identity_account::Result<Self> {
    Ok(Self {
      account: Account::builder().build().await?,
    })
  }
}

#[async_trait::async_trait]
impl RequestHandler for IdentityStorageHandler {
  type Request = StorageRequest;

  async fn handle(&mut self, request: Self::Request) -> identity_account::Result<<Self::Request as ActorRequest>::Response> {
    println!("Received {:?}", request);

    // TODO: PreProcessingHook

    let response = match request {
      StorageRequest::Create(req) => {
        let snapshot = self.account.create_identity(req).await?;

        let did = snapshot.identity().try_did()?;

        let document = self.account.resolve_identity(did).await?;

        StorageResponse::Create(document)
      }
      StorageRequest::List => {
        let list = self
          .account
          .list_identities()
          .await
          .iter()
          .map(|tag| self.account.find_identity(tag.method_id()))
          .collect::<FuturesOrdered<_>>()
          .try_collect::<Vec<_>>()
          .await?
          .into_iter()
          .filter_map(convert::identity)
          .map(|snapshot| snapshot.identity().to_document())
          .collect::<identity_account::Result<Vec<_>>>()?;

        StorageResponse::List(list)
      }
      _ => todo!(),
    };

    println!("Returning: {:?}", response);

    Ok(response)

    // TODO: PostProcessingHook
  }
}
