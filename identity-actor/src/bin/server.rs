// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{str::FromStr, sync::Arc};

use futures::future::join;
use identity_account::{
  identity::IdentityId,
  types::{Generation, KeyLocation},
};
use identity_actor::{
  communicator::DefaultIdentityCommunicator,
  types::{IdentityStorageRequest, IdentityStorageResponse},
  DefaultIdentityHandler,
};
use identity_did::verification::MethodType;
use libp2p::PeerId;

#[async_std::main]
async fn main() {
  let handler = DefaultIdentityHandler::new();
  let mut comm = DefaultIdentityCommunicator::new(handler).await;
  let addr = comm.start_listening(None).await;

  let shared_comm = Arc::new(comm);
  let shared_clone = Arc::clone(&shared_comm);

  let request_handle = async_std::task::spawn(async move { shared_clone.handle_requests().await });

  // Handle can still be used to send commands from another task/thread
  let send_handle = async_std::task::spawn(async move {
    shared_comm
      .send_command::<IdentityStorageResponse, _>(
        addr,
        PeerId::from_str("12D3KooWQb2MDHhqhXj5cgnprciKHUTpcLQkD6dSawtkXVDQQmdS").unwrap(),
        IdentityStorageRequest::KeyNew {
          id: IdentityId::from_u32(0),
          location: KeyLocation::new_authentication(MethodType::Ed25519VerificationKey2018, Generation::new()),
        },
      )
      .await
  });

  let (req_res, send_res) = join(request_handle, send_handle).await;

  println!("{:?}, {:?}", req_res, send_res);
}
