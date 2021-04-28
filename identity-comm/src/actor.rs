// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::riker::system::Run;
use crate::{
  envelope::{self, Encrypted, EncryptionAlgorithm, SignatureAlgorithm},
  message::{DidRequest, DidResponse, Message, Trustping, TrustpingResponse},
};
use futures_util::future::RemoteHandle;
use identity_account::{account::Account, storage::Storage};
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


// Apparently we need to use dynamic dispatch to get around a generic DidCommActor<T: DidCommunicator>. The workaround
// seems to be needed, since the #actor macro does not seem to respect a generic type parameter. (Actor::Msg needs to be a struct or Enum). Not clear if this really is the case, did https://github.com/riker-rs/riker/pull/124 solve another issue?
pub struct DidCommActor<S: Storage> {
  account: Arc<Account<S>>,
}

impl<S: 'static + Storage + Send + Sync>
  ActorFactoryArgs<Arc<Account<S>>> for DidCommActor<S>
{
  fn create_args(config: Arc<Account<S>>) -> Self {
    Self {
      account: config,
    }
  }
}

impl<S: Storage> DidCommActor<S> {
  fn new(account: Account<S>) -> Self {
    Self {
      account: Arc::new(account),
    }
  }
}

impl<S: Storage> Receive<Trustping> for DidCommActor<S> {
  type Msg = Request;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: Trustping, sender: Sender) {
    println!("trustping received");
    sender
      .expect("Sender should exists")
      // TODO get rid of the Response enum?!
      .try_tell(Response::Trustping(TrustpingResponse::default()), None)
      .expect("Sender should receive the response");
  }
}

impl<S: Storage + Send + Sync> Receive<DidRequest> for DidCommActor<S> {
  type Msg = Request;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DidRequest, sender: Sender) {
    let did = async_std::task::block_on(async {
      if let Some(id) = msg.id() {
        self.account.get(id).await
      } else {
        let chain = self.account.try_with_index(|index| index.try_first())?;
        self.account.get(chain).await
      }
    })
    .unwrap();

    let response = Response::Did(DidResponse::new(did.id().clone()));
    sender
      .expect("sender should exist")
      .try_tell(response, ctx.myself())
      .expect("could not send")
  }
}

impl<S: 'static + Storage + Send + Sync> Actor for DidCommActor<S> {
  type Msg = Request;

  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // Use the respective Receive<T> implementation
    match msg {
      Request::Trustping(msg) => Receive::<Trustping>::receive(self, ctx, msg, sender),
      Request::Did(msg) => Receive::<DidRequest>::receive(self, ctx, msg, sender),
    }
  }
}

/* -------------------------------------------------------------------------- */
/* ENCRYPTED ACTOR */
/* -------------------------------------------------------------------------- */

/// An actor that wraps communication of another actor in an [Encrypted] envelope
pub struct EncryptedActor<I: riker::Message, O> {
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
    let response_envelope = Self::Msg::pack(&response, self.algorithm, &[self.recipients.clone()], &self.keypair)
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

/// An actor that wraps communication of another actor in an [Encrypted] envelope
pub struct SignedActor<I: riker::Message, O> {
  inner: ActorRef<I>,
  recipients: PublicKey,
  keypair: KeyPair,
  algorithm: SignatureAlgorithm,
  _output: PhantomData<O>,
}

/* -------------------------------------------------------------------------- */
/* SIGNED ACTOR */
/* -------------------------------------------------------------------------- */

// I: Request, O: Response
impl<I, O> Actor for SignedActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  type Msg = envelope::Signed;
  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // unpack request
    let msg = msg
      .unpack::<I>(self.algorithm, &self.recipients)
      .expect("could not unpack message");

    // pass unpacked request to inner actor and await response
    let handle: RemoteHandle<O> = ask::<I, _, _, _>(&ctx.system.clone(), &self.inner, msg);
    let response: O = ctx
      .run(handle)
      .map(|fut| async_std::task::block_on(async { fut.await }))
      .expect("could not run");

    // pack response
    let response_envelope = Self::Msg::pack(&response, self.algorithm, &self.keypair).expect("could not pack message");

    sender
      .expect("sender must be present")
      .try_tell(response_envelope, ctx.myself())
      .expect("could not send");
  }
}

impl<I, O> ActorFactoryArgs<(ActorRef<I>, PublicKey, KeyPair, SignatureAlgorithm)> for SignedActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  fn create_args(config: (ActorRef<I>, PublicKey, KeyPair, SignatureAlgorithm)) -> Self {
    Self {
      inner: config.0,
      recipients: config.1,
      keypair: config.2,
      algorithm: config.3,
      _output: PhantomData::default(),
    }
  }
}

/* -------------------------------------------------------------------------- */
/* PLAINTEXT ACTOR */
/* -------------------------------------------------------------------------- */

/// An actor that wraps communication of another actor in an [Encrypted] envelope
pub struct PlaintextActor<I: riker::Message, O> {
  inner: ActorRef<I>,
  _output: PhantomData<O>,
}

// I: Request, O: Response
impl<I, O> Actor for PlaintextActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  type Msg = envelope::Plaintext;
  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // unpack request
    let msg = msg.unpack::<I>().expect("could not unpack message");

    // pass unpacked request to inner actor and await response
    let handle: RemoteHandle<O> = ask::<I, _, _, _>(&ctx.system.clone(), &self.inner, msg);
    let response: O = ctx
      .run(handle)
      .map(|fut| async_std::task::block_on(async { fut.await }))
      .expect("could not run");

    // pack response
    let response_envelope = Self::Msg::pack(&response).expect("could not pack message");

    sender
      .expect("sender must be present")
      .try_tell(response_envelope, ctx.myself())
      .expect("could not send");
  }
}

impl<I, O> ActorFactoryArgs<(ActorRef<I>)> for PlaintextActor<I, O>
where
  I: riker::Message + Message,
  O: riker::Message + Message + Serialize,
  for<'de> I: Deserialize<'de>,
{
  fn create_args(config: ActorRef<I>) -> Self {
    Self {
      inner: config,
      _output: PhantomData::default(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_my_communicator() {
    // let _actor = DidCommActor::new(MyCommunicator);
  }
}

