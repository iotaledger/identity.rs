// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::riker::system::Run;
use crate::{
  envelope::{self, Encrypted, EncryptionAlgorithm},
  message::{DidRequest, DidResponse, Message, Trustping, TrustpingResponse},
};
use futures_util::future::RemoteHandle;
use identity_core::crypto::{KeyPair, PublicKey, SecretKey};
use identity_iota::did::Document;
use riker::actor::BasicActorRef;
use riker::actor::Context;
use riker::actor::Receive;
use riker::actor::Sender;
use riker::actor::{actor, ActorRef};
use riker::actor::{Actor, ActorFactoryArgs};
use riker_patterns::ask::ask;
use serde::{Deserialize, Serialize};
use std::{
  any::{Any, TypeId},
  collections::HashMap,
  fmt::Debug,
  marker::PhantomData,
  sync::{Arc, Mutex},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
  Trustping(Trustping),
  Did(DidRequest),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
  Trustping(TrustpingResponse),
  Did(DidResponse),
}

impl From<Trustping> for Request {
  fn from(other: Trustping) -> Self {
    Request::Trustping(other)
  }
}
impl From<DidRequest> for Request {
  fn from(other: DidRequest) -> Self {
    Request::Did(other)
  }
}

// Communicator trait specifies all default request->response mappings
// The drawback is that we can't refer to fields
pub trait DidCommunicator {
  type Msg: riker::Message;

  fn receive_trustping(&mut self, _ctx: &Context<Self::Msg>, _msg: Trustping, sender: Sender) {
    println!("trustping received");
    sender
      .expect("Sender should exists")
      // TODO get rid of the Response enum?!
      .try_tell(Response::Trustping(TrustpingResponse::default()), None)
      .expect("Sender should receive the response");
  }

  fn receive_did_request(&mut self, _ctx: &Context<Self::Msg>, _msg: DidRequest, sender: Sender) {
    println!("didrequest received");
    sender
      .expect("Sender should exists")
      .try_tell(
        Response::Did(DidResponse::new("did:example:123".parse().unwrap())),
        None,
      )
      .expect("Sender should receive the response");
  }
}

// Apparently we need to use dynamic dispatch to get around a generic DidCommActor<T: DidCommunicator>. The workaround
// seems to be needed, since the #actor macro does not seem to respect a generic type parameter. (Actor::Msg needs to be a struct or Enum). Not clear if this really is the case, did https://github.com/riker-rs/riker/pull/124 solve another issue?
pub struct DidCommActor {
  actor: Box<dyn DidCommunicator<Msg = Request> + Send>,
}

impl DidCommActor {
  fn new<T: 'static + DidCommunicator<Msg = Request> + Send>(actor: T) -> Self {
    Self { actor: Box::new(actor) }
  }
}

impl Default for DidCommActor {
  fn default() -> Self {
    Self::new(DefaultCommunicator)
  }
}

impl Receive<Trustping> for DidCommActor {
  type Msg = Request;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: Trustping, sender: Sender) {
    self.actor.receive_trustping(ctx, msg, sender);
  }
}

impl Receive<DidRequest> for DidCommActor {
  type Msg = Request;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DidRequest, sender: Sender) {
    self.actor.receive_did_request(ctx, msg, sender)
  }
}

impl Actor for DidCommActor {
  type Msg = Request;

  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // Use the respective Receive<T> implementation
    match msg {
      Request::Trustping(msg) => Receive::<Trustping>::receive(self, ctx, msg, sender),
      Request::Did(msg) => Receive::<DidRequest>::receive(self, ctx, msg, sender),
    }
  }
}

pub struct DefaultCommunicator;
impl DidCommunicator for DefaultCommunicator {
  type Msg = Request;
}

/// An actor that wraps communication of another actor in an [Encrypted] envelope
pub struct EncryptedActor<I: riker::Message, O> {
  // this prevents from wrapping signed envelopes in encrypted envelopes
  inner: ActorRef<I>,
  recipients: PublicKey,
  keypair: KeyPair,
  algorithm: EncryptionAlgorithm,
  _output: PhantomData<O>,
}

// I: Request, O: Response
impl<I, O> Actor for EncryptedActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  type Msg = envelope::Encrypted;
  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // unpack request
    let msg = msg
      .unpack::<I>(self.algorithm, &self.keypair.secret(), &self.recipients)
      .expect("could not unpack message");

    // pass unpacked request to inner actor and await response
    let handle: RemoteHandle<O> = ask::<I, _, _, _>(&ctx.system.clone(), &self.inner, msg);
    let response: O = ctx
      .run(handle)
      .map(|fut| async_std::task::block_on(async { fut.await }))
      .expect("could not run");

    // pack response
    let response_envelope = Encrypted::pack(&response, self.algorithm, &[self.recipients.clone()], &self.keypair)
      .expect("could not pack message");

    sender
      .expect("sender must be present")
      .try_tell(response_envelope, ctx.myself())
      .expect("could not send");
  }
}

impl<I, O> ActorFactoryArgs<(ActorRef<I>, PublicKey, KeyPair, EncryptionAlgorithm)> for EncryptedActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  fn create_args(config: (ActorRef<I>, PublicKey, KeyPair, EncryptionAlgorithm)) -> Self {
    Self {
      inner: config.0,
      recipients: config.1,
      keypair: config.2,
      algorithm: config.3,
      _output: PhantomData::default(),
    }
  }
}

// !  overwriting handlers requires a tiny bit of boilerplate

/// Custom communicator that overwrites receive_trustping
pub struct MyCommunicator;
impl DidCommunicator for MyCommunicator {
  type Msg = Request;

  fn receive_trustping(&mut self, _ctx: &Context<Request>, _msg: Trustping, sender: Sender) {
    dbg!("trustping received - custom response");
    sender
      .expect("Sender should exists")
      .try_tell(TrustpingResponse::default(), None)
      .expect("Sender should receive the response");
  }
}

// ! Implementing a custom actor that has custom fields and overwrites the trustping handler.
// ! It takes the default DidRequest handler (requires some boilerplate per Default-request handled)

#[actor(Trustping, DidRequest)]
pub struct MyActor {
  my_state: bool,
}

impl Actor for MyActor {
  // we used the #[actor] attribute so MyActorMsg is the Msg type
  type Msg = MyActorMsg;

  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // Use the respective Receive<T> implementation
    self.receive(ctx, msg, sender);
  }
}

impl DidCommunicator for MyActor {
  type Msg = MyActorMsg;
}

/// impl custom behavior for trustpings
/// we can insert behavior before and after the default call or implement a whole new method
impl Receive<Trustping> for MyActor {
  type Msg = MyActorMsg;
  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: Trustping, sender: Sender) {
    dbg!("trustping received - custom conditional response");
    if self.my_state {
      self.receive_trustping(ctx, msg, sender);
    } else {
      dbg!("do something else");
    }
  }
}

/// Using default behavior boiler plate for DidRequest
impl Receive<DidRequest> for MyActor {
  type Msg = MyActorMsg;
  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DidRequest, sender: Sender) {
    self.receive_did_request(ctx, msg, sender);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_my_communicator() {
    let _actor = DidCommActor::new(MyCommunicator);
  }
}

use identity_account::account::Account;
use identity_account::storage::MemStore;
use identity_account::storage::Storage;

struct Acti {
  account: Arc<Account<MemStore>>, //account: Account<S>
}

use identity_iota::did::DID;

// #[actor(DidRequest)]
// pub struct MyActor2 {
//   account: Arc<Account<MemStore>>,
// }

// impl Actor for MyActor2 {
//   type Msg = MyActor2Msg;

//   fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
//     let did: &DID = todo!();
//     let account: Arc<Account<_>> = Arc::clone(&self.account);

//     ctx.run(async move {
//       account.get(did.clone()).await;
//     });
//   }
// }

// use identity_account::{account::Account, storage::MemStore};
// use riker::actor::actor;
// use riker::actor::{Actor, BasicActorRef, Context, Receive, Sender};
// use riker::system::Run;
// use identity_iota::did::DID;

// #[derive(Clone, Debug)]
// pub struct DidRequest {

// }

// impl Receive<DidRequest> for MyActor {
//   type Msg = MyActorMsg;

//   fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DidRequest, sender: Sender) {

//   }
// }

// use std::sync::Arc;

// #[actor(DidRequest)]
// pub struct MyActor {
//   account: Arc<Account<MemStore>>,
// }

// impl Actor for MyActor {
//   type Msg = MyActorMsg;

//   fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
//     let did: &DID = todo!();
//     let account: Arc<Account<_>> = Arc::clone(&self.account);

//     ctx.run(async move {
//       account.get(did.clone()).await;
//     });
//   }
// }
