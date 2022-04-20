// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
use identity_did::verification::MethodScope;
use identity_iota::document::ResolvedIotaDocument;
use identity_iota::tangle::Resolver;
use identity_iota_core::did::IotaDIDUrl;
use identity_iota_core::document::IotaVerificationMethod;
use libp2p::PeerId;

use crate::actor::ActorRequest;
use crate::actor::Asynchronous;
use crate::actor::Endpoint;
use crate::actor::RequestContext;
use crate::actor::Result as ActorResult;
use crate::didcomm::message::EmptyMessage;
use crate::didcomm::DIDCommKeyConfig;

use super::message::DidCommPlaintextMessage;
use super::state::DidCommState;
use super::thread_id::ThreadId;
use super::DidCommActor;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
  recipient_key: Option<UrlOrKey>,
  #[serde(default)]
  routing_keys: Vec<UrlOrKey>,
}

impl Connection {
  pub fn new() -> Self {
    Self {
      recipient_key: None,
      routing_keys: Vec::new(),
    }
  }

  pub fn set_recipient_key(&mut self, recipient_key: UrlOrKey) {
    self.recipient_key = Some(recipient_key);
  }
}

impl Default for Connection {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DIDKey(String);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum UrlOrKey {
  DIDUrl(IotaDIDUrl),
  Key(DIDKey),
}

impl ActorRequest<Asynchronous> for Connection {
  type Response = ();

  fn endpoint() -> Endpoint {
    "didcomm/connection".parse().unwrap()
  }
}

// TODO: Incomplete
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
  recipient_keys: Option<UrlOrKey>,
}

impl DidCommState {
  pub async fn connection(self, mut actor: DidCommActor, request: RequestContext<DidCommPlaintextMessage<Connection>>) {
    match request.input.body.recipient_key {
      Some(UrlOrKey::DIDUrl(ref peer_key)) => {
        // TODO: How is *this* actor supposed to know, which of their own keys is used for encryption?
        // It could be any of the `recipientKeys` sent in the invitation.
        // Just use `kex-0` for now.
        let own_key: IotaDIDUrl = actor
          .state
          .identity
          .doc
          .resolve_method("kex-0", Some(MethodScope::key_agreement()))
          .unwrap()
          .id()
          .to_owned();

        let resolver: Resolver = Resolver::new().await.expect("TODO");
        let peer_doc: ResolvedIotaDocument = resolver.resolve(peer_key.did()).await.expect("TODO");

        println!("resolved peer did {}", peer_doc.document.id());

        let method: &IotaVerificationMethod = peer_doc
          .document
          .resolve_method(peer_key, Some(MethodScope::key_agreement()))
          .unwrap();
        let public_key: PublicKey = PublicKey::from(method.data().try_decode().expect("TODO"));

        println!(
          "setting up encryption for peer {} on actor {}",
          request.peer,
          actor.state.identity.doc.id()
        );

        // ACK we're done setting up encryption.
        actor
          .send_message(
            request.peer,
            request.input.thread_id(),
            super::message::EmptyMessage::new(),
          )
          .await
          .expect("TODO");

        // Activate encryption after sending the ack, otherwise ack is encrypted.
        actor
          .state
          .didcomm_config
          .peer_keys
          .insert(request.peer, DIDCommKeyConfig::new(own_key, public_key));
      }
      Some(UrlOrKey::Key(_)) => unimplemented!("did:key"),
      None if self.require_anon_crypt => {
        todo!("send problem report: no recipient keys provided but anon crypt is required");
      }
      None => (),
    }
  }
}

pub async fn accept_invitation(
  actor: &mut DidCommActor,
  peer_id: PeerId,
  own_key_url: IotaDIDUrl,
  peer_key_url: IotaDIDUrl,
) -> ActorResult<()> {
  let resolver: Resolver = Resolver::new().await.expect("TODO");
  let peer_doc: ResolvedIotaDocument = resolver.resolve(peer_key_url.did()).await.expect("TODO");

  let thread_id = ThreadId::new();
  let recv_method: &IotaVerificationMethod = peer_doc
    .document
    .resolve_method(peer_key_url, Some(MethodScope::key_agreement()))
    .unwrap();

  let mut connection = Connection::new();
  // Set the did url of our key which we want the peer to use for encryption.
  connection.set_recipient_key(UrlOrKey::DIDUrl(own_key_url.clone()));

  actor.send_message(peer_id, &thread_id, connection).await.unwrap();

  let _ack: DidCommPlaintextMessage<EmptyMessage> = actor.await_message(&thread_id).await.unwrap();

  actor.state.didcomm_config.peer_keys.insert(
    peer_id,
    DIDCommKeyConfig::new(
      own_key_url,
      PublicKey::from(recv_method.data().try_decode().expect("TODO")),
    ),
  );

  Ok(())
}
