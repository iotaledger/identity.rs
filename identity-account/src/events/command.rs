// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::DID;

use crate::chain::ChainData;
use crate::chain::ChainKey;
use crate::events::Context;
use crate::events::Event;
use crate::error::Result;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Index;
use crate::types::Fragment;
use crate::types::Timestamp;

macro_rules! ensure {
  ($cond:expr, $message:literal $(,)?) => {
    if !$cond {
      return Err($crate::Error::InvalidCommandContext($message));
    }
  };
}

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
        ensure!(state.document().is_none(), "Invalid Chain State: Chain Exists",);

        ensure!(
          !matches!(authentication, MethodType::MerkleKeyCollection2021),
          "Invalid Method Type: MerkleKeyCollection2021"
        );

        let location: ChainKey = ChainKey::auth(authentication, Index::ZERO);

        trace!("[Command::process] Chain Key = {:#?}", location);

        ensure!(
          store.key_get(chain, &location).await.is_err(),
          "Invalid Chain State: Duplicate Authentication Method",
        );

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
        ensure!(state.document().is_some(), "Invalid Chain State: Chain Not Initialized",);

        ensure!(fragment != ChainKey::AUTH, "Invalid Method Fragment: Reserved",);

        let location: ChainKey = ChainKey {
          type_,
          auth: state.auth_index(),
          diff: state.diff_index(),
          fragment: Fragment::new(fragment),
        };

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
