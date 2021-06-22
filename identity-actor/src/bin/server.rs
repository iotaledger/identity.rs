// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_actor::{communicator::DefaultIdentityCommunicator, DefaultIdentityHandler};

#[async_std::main]
async fn main() {
  let handler = DefaultIdentityHandler::new();
  let mut comm = DefaultIdentityCommunicator::new(handler).await;
  comm.start_listening(None).await;
}
