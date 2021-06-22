// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use communication_refactored::{Multiaddr, PeerId};
use identity_account::{
  identity::IdentityId,
  types::{Generation, KeyLocation},
};
use identity_actor::{
  communicator::DefaultIdentityCommunicator, types::IdentityStorageRequest, DefaultIdentityHandler,
};
use identity_did::verification::MethodType;
use std::str::FromStr;

#[async_std::main]
async fn main() {
  let args: Vec<String> = std::env::args().collect();

  let address = &args[1];
  let peer_id = &args[2];

  println!("Connecting to {:?} with id: {:?}", address, peer_id);

  let handler = DefaultIdentityHandler::new();
  let mut comm = DefaultIdentityCommunicator::new(handler).await;

  let response = comm
    .send_command(
      Multiaddr::from_str(address).unwrap(),
      PeerId::from_str(peer_id).unwrap(),
      IdentityStorageRequest::KeyNew {
        id: IdentityId::from_u32(0),
        location: KeyLocation::new_authentication(MethodType::Ed25519VerificationKey2018, Generation::new()),
      },
    )
    .await;

  println!("Received: {:?}", response);
}
