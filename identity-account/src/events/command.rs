// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::DID;

use crate::chain::ChainData;
use crate::chain::ChainKey;
use crate::error::Result;
use crate::events::Context;
use crate::events::Event;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Fragment;
use crate::types::Index;
use crate::types::Timestamp;

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
  CreateChain {
    network: Option<String>,
    shard: Option<String>,
    authentication: MethodType,
  },
  CreateMethod {
    type_: MethodType,
    scope: MethodScope,
    fragment: String,
  },
}

impl_command_builder!(CreateChain {
  @optional network String,
  @optional shard String,
  @required authentication MethodType,
});

impl_command_builder!(CreateMethod {
  @required type_ MethodType,
  @default  scope MethodScope,
  @required fragment String,
});

impl Command {
  pub async fn process<T>(self, context: Context<'_, T>) -> Result<Option<Vec<Event>>>
  where
    T: Storage,
  {
    let state: &ChainData = context.state();
    let store: &T = context.store();
    let chain: ChainId = state.chain();

    debug!("[Command::process] Chain   = {:#?}", chain);
    debug!("[Command::process] Command = {:#?}", self);
    trace!("[Command::process] State   = {:#?}", state);
    trace!("[Command::process] Store   = {:#?}", store);

    match self {
      Self::CreateChain {
        network,
        shard,
        authentication,
      } => {
        assert_new_document(state)?;
        assert_auth_type(authentication)?;

        let location: ChainKey = ChainKey::auth(authentication, Index::ZERO);

        trace!("[Command::process] Chain Key = {:#?}", location);

        assert_key_blank::<T>(chain, store, &location).await?;

        // Generate a private key at the initial auth index (`0`)
        let public: PublicKey = store.key_new(chain, &location).await?;

        // Generate a new DID URL from the public key
        let network: Option<&str> = network.as_deref();
        let shard: Option<&str> = shard.as_deref();
        let document: DID = DID::from_components(public.as_ref(), network, shard)?;

        Event::respond_one(Event::ChainCreated {
          document,
          timestamp: Timestamp::now(),
        })
      }
      Self::CreateMethod { type_, scope, fragment } => {
        assert_old_document(state)?;
        assert_frag_reserved(&fragment)?;

        let location: ChainKey = ChainKey {
          type_,
          auth: state.auth_index(),
          diff: state.diff_index().try_increment()?,
          fragment: Fragment::new(fragment),
        };

        assert_key_blank::<T>(chain, store, &location).await?;

        trace!("[Command::process] Chain Key = {:#?}", location);

        let _public: PublicKey = store.key_new(chain, &location).await?;

        Event::respond_one(Event::MethodCreated {
          scope,
          location,
          timestamp: Timestamp::now(),
        })
      }
    }
  }
}

fn assert_new_document(state: &ChainData) -> Result<()> {
  ensure!(state.document().is_none(), "Document Already Exists");
  Ok(())
}

fn assert_old_document(state: &ChainData) -> Result<()> {
  ensure!(state.document().is_some(), "Document Not Found");
  Ok(())
}

fn assert_auth_type(type_: MethodType) -> Result<()> {
  ensure!(
    !matches!(type_, MethodType::MerkleKeyCollection2021),
    "Type Not Allowed - MerkleKeyCollection2021"
  );

  Ok(())
}

fn assert_frag_reserved(fragment: &str) -> Result<()> {
  ensure!(fragment != ChainKey::AUTH, "Fragment Not Allowed - Reserved");
  Ok(())
}

async fn assert_key_blank<T: Storage>(chain: ChainId, store: &T, location: &ChainKey) -> Result<()> {
  ensure!(store.key_get(chain, location).await.is_err(), "Duplicate Key Location");
  Ok(())
}
