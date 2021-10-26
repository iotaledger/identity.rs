// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use libp2p::Multiaddr;
use libp2p::PeerId;

use crate::storage::requests::IdentityList;
use crate::Actor;
use crate::IdentityResolve;
use crate::Result;
use crate::StorageHandler;

use super::default_listening_actor;
use super::default_sending_actor;

async fn default_storage_listening_actor() -> Result<(Actor, Multiaddr, PeerId)> {
  let (mut listening_actor, addr, peer_id) = default_listening_actor().await;

  let handler = StorageHandler::new().await.unwrap();

  listening_actor
    .add_state(handler)
    .add_handler("storage/list", StorageHandler::list)?;
  // .add_method("storage/resolve", StorageHandler::resolve)?;

  Ok((listening_actor, addr, peer_id))
}

#[tokio::test]
async fn test_list_identities() -> Result<()> {
  let (listening_actor, addr, peer_id) = default_storage_listening_actor().await?;

  let mut sending_actor = default_sending_actor().await;
  sending_actor.add_peer(peer_id, addr).await;

  let result = sending_actor.send_request(peer_id, IdentityList).await?;

  assert!(result.is_empty());

  listening_actor.stop_handling_requests().await.unwrap();

  Ok(())
}

#[tokio::test]
async fn test_resolve_on_unknown_id_returns_err() -> Result<()> {
  let (listening_actor, addr, peer_id) = default_storage_listening_actor().await?;

  let mut sending_actor = default_sending_actor().await;
  sending_actor.add_peer(peer_id, addr).await;

  let result = sending_actor
    .send_request(
      peer_id,
      IdentityResolve::new("did:iota:FFFAH6qct9KGQcSenG1iaw2Nj9jP7Zmug2zcmTpF4942".parse().unwrap()),
    )
    .await;

  assert!(result.unwrap().is_err());

  listening_actor.stop_handling_requests().await.unwrap();

  Ok(())
}
