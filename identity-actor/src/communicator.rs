// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::errors::{Error, Result};
use communication_refactored::{
  firewall::{FirewallConfiguration, PermissionValue, ToPermissionVariants, VariantPermission},
  Keypair, NetworkEvent, ReceiveRequest, RequestMessage, ShCommunication, ShCommunicationBuilder,
};
use communication_refactored::{InitKeypair, Multiaddr, PeerId, RqRsMessage};
use futures::{channel::mpsc, lock::Mutex, StreamExt};
use identity_macros::IdentityHandler;
use libp2p::tcp::TcpConfig;
use std::fmt;

use crate::{handler::IdentityStorageHandler, IdentityRequestHandler};

pub struct IdentityCommunicator<Req, Res, ReqPerm, ReqHandler>
where
  Req: fmt::Debug + RqRsMessage + ToPermissionVariants<ReqPerm>,
  Res: fmt::Debug + RqRsMessage,
  ReqPerm: VariantPermission,
  ReqHandler: IdentityRequestHandler<Request = Req, Response = Res>,
{
  net_events_receiver: mpsc::Receiver<NetworkEvent>,
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
    let (net_events_sender, net_events_receiver) = mpsc::channel(1);
    let comm = ShCommunicationBuilder::new(dummy_tx, rq_tx, Some(net_events_sender))
      .with_firewall_config(FirewallConfiguration::allow_all())
      .with_keys(InitKeypair::IdKeys(id_keys))
      .build_with_transport(transport)
      .await;

    Self {
      net_events_receiver,
      comm,
      receiver_handler: Mutex::new((handler, rq_rx)),
    }
  }

  pub async fn start_listening(&mut self, address: Option<Multiaddr>) -> Multiaddr {
    let addr = self.comm.start_listening(address).await.unwrap();

    println!("{} {}", addr, self.comm.get_peer_id());

    addr
  }

  pub async fn handle_requests(&self) -> Result<()> {
    let mut handler_receiver = self.receiver_handler.try_lock().ok_or(Error::LockInUse)?;

    loop {
      let ReceiveRequest {
        peer: _,
        request_id: _,
        request: RequestMessage { response_tx, data },
      } = handler_receiver.1.next().await.unwrap();

      let response = handler_receiver.0.handle(data);

      response_tx.send(response).unwrap();
    }
  }

  pub async fn send_command(&mut self, addr: Multiaddr, peer: PeerId, command: impl Into<Req>) -> Res {
    self.comm.add_address(peer, addr);
    let recv = self.comm.send_request(peer, command.into());
    match recv.response_rx.await {
      Ok(res) => res,
      Err(err) => {
        println!("{:?}", err);
        let ev = self.net_events_receiver.next().await;
        println!("{:#?}", ev);
        todo!()
      }
    }
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
