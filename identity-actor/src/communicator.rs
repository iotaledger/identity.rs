// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{Error, Result};
use communication_refactored::{
  firewall::{FirewallConfiguration, PermissionValue, ToPermissionVariants, VariantPermission},
  Keypair, ReceiveRequest, ShCommunication, ShCommunicationBuilder,
};
use communication_refactored::{InitKeypair, Multiaddr, PeerId, RqRsMessage};
use futures::{channel::mpsc, lock::Mutex, StreamExt};
use identity_macros::IdentityHandler;
use libp2p::tcp::TcpConfig;
use std::{convert::TryInto, fmt};

use crate::{handler::IdentityStorageHandler, IdentityRequestHandler};

pub struct IdentityCommunicator<Req, Res, ReqPerm, ReqHandler>
where
  Req: fmt::Debug + RqRsMessage + ToPermissionVariants<ReqPerm>,
  Res: fmt::Debug + RqRsMessage,
  ReqPerm: VariantPermission,
  ReqHandler: IdentityRequestHandler<Request = Req, Response = Res>,
{
  comm: ShCommunication<Req, Res, ReqPerm>,
  receiver_handler: Mutex<(ReqHandler, mpsc::Receiver<ReceiveRequest<Req, Res>>)>,
}

impl<Req, Res, ReqPerm, ReqHandler> IdentityCommunicator<Req, Res, ReqPerm, ReqHandler>
where
  Req: fmt::Debug + RqRsMessage + ToPermissionVariants<ReqPerm>,
  Res: fmt::Debug + RqRsMessage,
  ReqPerm: VariantPermission,
  ReqHandler: IdentityRequestHandler<Request = Req, Response = Res>,
{
  pub async fn new(handler: ReqHandler) -> Self {
    let id_keys = Keypair::generate_ed25519();

    let transport = TcpConfig::new().nodelay(true);
    let (dummy_tx, _) = mpsc::channel(1);
    let (rq_tx, rq_rx) = mpsc::channel(1);
    let comm = ShCommunicationBuilder::new(dummy_tx, rq_tx, None)
      .with_firewall_config(FirewallConfiguration::allow_all())
      .with_keys(InitKeypair::IdKeys(id_keys))
      .build_with_transport(transport)
      .await;

    Self {
      comm,
      receiver_handler: Mutex::new((handler, rq_rx)),
    }
  }

  pub async fn start_listening(&mut self, address: Option<Multiaddr>) -> Multiaddr {
    let addr = self.comm.start_listening(address).await.unwrap();

    println!("{} {}", addr, self.comm.get_peer_id());

    addr
  }

  /// Start handling incoming requests. This method does not return unless [`stop_listening`] is called.
  /// This method should only be called once on any given instance.
  /// A second caller would immediately receive an [`Error::LockInUse`].
  pub async fn handle_requests(&self) -> Result<()> {
    let mut handler_receiver = self.receiver_handler.try_lock().ok_or(Error::LockInUse)?;

    loop {
      let ReceiveRequest {
        peer: _,
        request_id: _,
        response_tx,
        request,
      } = handler_receiver.1.next().await.expect("Is only called on shutdown");

      let response = handler_receiver.0.handle(request);

      response_tx.send(response).unwrap();
    }
  }

  pub async fn send_command<Ret, Cmd>(&self, addr: Multiaddr, peer: PeerId, command: Cmd) -> Result<Ret>
  where
    Res: TryInto<Ret, Error = crate::Error>,
    Cmd: Into<Req>,
  {
    self.comm.add_address(peer, addr);
    let recv = self.comm.send_request(peer, command.into());
    let response = recv.response_rx.await.unwrap()?;

    response.try_into()
  }
}

use crate as identity_actor;
#[derive(IdentityHandler)]
pub struct DefaultIdentityHandler {
  identity_storage_handler: IdentityStorageHandler,
}

impl DefaultIdentityHandler {
  pub fn new() -> Self {
    Self {
      identity_storage_handler: IdentityStorageHandler::new(),
    }
  }
}

/// The default communicator that handles storage and DIDComm requests
pub use CustomIdentityCommunicator as DefaultIdentityCommunicator;
