// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{errors::Error, IdentityResolve};

use super::{default_listening_actor, default_sending_actor};

#[tokio::test]
async fn test_unknown_request() -> anyhow::Result<()> {
  pretty_env_logger::init();

  let (listening_actor, addr, peer_id) = default_listening_actor().await;

  let mut sending_actor = default_sending_actor().await;
  sending_actor.add_peer(peer_id, addr).await;

  let request_name = "unknown/request";

  let result = sending_actor
    .send_named_request(
      peer_id,
      request_name,
      IdentityResolve::new("did:iota:FFFAH6qct9KGQcSenG1iaw2Nj9jP7Zmug2zcmTpF4942".parse().unwrap()),
    )
    .await;

  assert!(matches!(result.unwrap_err(), Error::UnknownRequest(_)));

  listening_actor.stop_handling_requests().await.unwrap();

  Ok(())
}
