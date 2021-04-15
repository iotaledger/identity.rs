// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Object;
use identity_core::common::Url;
use identity_core::crypto::PublicKey;
use identity_did::verification::MethodData;
use identity_did::verification::MethodScope;
use identity_did::verification::MethodType;
use identity_iota::did::DID;

use crate::chain::ChainData;
use crate::chain::ChainKey;
use crate::chain::TinyMethod;
use crate::chain::TinyService;
use crate::error::Result;
use crate::events::CommandError;
use crate::events::Context;
use crate::events::Event;
use crate::storage::Storage;
use crate::types::ChainId;
use crate::types::Index;
use crate::types::Timestamp;

const AUTH_TYPES: &[MethodType] = &[MethodType::Ed25519VerificationKey2018];

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

impl_command_builder!(DeleteMethod {
  @required fragment String,
  @optional scope MethodScope,
});

impl_command_builder!(CreateService {
  @required fragment String,
  @required type_ String,
  @required endpoint Url,
  @optional properties Object,
});

impl_command_builder!(DeleteService {
  @required fragment String,
});

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
  DeleteMethod {
    fragment: String,
    scope: Option<MethodScope>,
  },
  CreateService {
    fragment: String,
    type_: String,
    endpoint: Url,
    properties: Option<Object>,
  },
  DeleteService {
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

    debug!("[Command::process] Chain = {:#?}", chain);
    debug!("[Command::process] Command = {:#?}", self);
    trace!("[Command::process] State = {:#?}", state);
    trace!("[Command::process] Store = {:#?}", store);

    match self {
      Self::CreateChain {
        network,
        shard,
        authentication,
      } => {
        ensure!(state.document().is_none(), CommandError::DocumentAlreadyExists);

        ensure!(
          AUTH_TYPES.contains(&authentication),
          CommandError::InvalidMethodType(authentication)
        );

        let location: ChainKey = ChainKey::auth(authentication, Index::ZERO);

        trace!("[Command::process] Chain Key = {:#?}", location);

        ensure!(
          !store.key_exists(chain, &location).await?,
          CommandError::DuplicateKeyLocation(location)
        );

        // Generate a private key at the initial auth index (`0`)
        let public: PublicKey = store.key_new(chain, &location).await?;
        let data: MethodData = MethodData::new_b58(public.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data);

        // Generate a new DID URL from the public key
        let network: Option<&str> = network.as_deref();
        let shard: Option<&str> = shard.as_deref();
        let document: DID = DID::from_components(public.as_ref(), network, shard)?;

        Event::respond_one(Event::ChainCreated {
          document,
          method,
          timestamp: Timestamp::now(),
        })
      }
      Self::CreateMethod { type_, scope, fragment } => {
        ensure!(state.document().is_some(), CommandError::DocumentNotFound);

        ensure!(
          fragment != ChainKey::AUTH,
          CommandError::InvalidMethodFragment("reserved")
        );

        let location: ChainKey = state.key(type_, fragment)?;

        trace!("[Command::process] Chain Key = {:#?}", location);

        ensure!(
          !store.key_exists(chain, &location).await?,
          CommandError::DuplicateKeyLocation(location)
        );

        ensure!(
          !state.methods().contains(location.fragment()),
          CommandError::DuplicateKeyFragment(location),
        );

        let pkey: PublicKey = store.key_new(chain, &location).await?;
        let data: MethodData = MethodData::new_b58(pkey.as_ref());
        let method: TinyMethod = TinyMethod::new(location, data);

        Event::respond_one(Event::MethodCreated {
          scope,
          method,
          timestamp: Timestamp::now(),
        })
      }
      Self::DeleteMethod { fragment, scope } => {
        ensure!(state.document().is_some(), CommandError::DocumentNotFound);

        ensure!(
          fragment != ChainKey::AUTH,
          CommandError::InvalidMethodFragment("reserved")
        );

        ensure!(state.methods().contains(&fragment), CommandError::MethodNotFound);

        Event::respond_one(Event::MethodDeleted {
          fragment,
          scope,
          timestamp: Timestamp::now(),
        })
      }
      Self::CreateService {
        fragment,
        type_,
        endpoint,
        properties,
      } => {
        ensure!(state.document().is_some(), CommandError::DocumentNotFound);

        ensure!(
          !state.services().contains(&fragment),
          CommandError::DuplicateServiceFragment(fragment),
        );

        let service: TinyService = TinyService::new(fragment, type_, endpoint, properties);

        Event::respond_one(Event::ServiceCreated {
          service,
          timestamp: Timestamp::now(),
        })
      }
      Self::DeleteService { fragment } => {
        ensure!(state.document().is_some(), CommandError::DocumentNotFound);
        ensure!(state.services().contains(&fragment), CommandError::ServiceNotFound);

        Event::respond_one(Event::ServiceDeleted {
          fragment,
          timestamp: Timestamp::now(),
        })
      }
    }
  }
}
