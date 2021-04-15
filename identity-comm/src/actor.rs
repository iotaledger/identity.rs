use crate::{
  envelope::{self, Encrypted, EncryptionAlgorithm},
  message::{DidRequest, DidResponse, Message, Trustping, TrustpingResponse},
};
use futures_util::future::RemoteHandle;
use identity_core::crypto::{KeyPair, PublicKey, SecretKey};
use riker::actor::BasicActorRef;
use riker::actor::Context;
use riker::actor::Receive;
use riker::actor::Sender;
use riker::actor::{actor, ActorRef};
use riker::actor::{Actor, ActorFactoryArgs};
use serde::Deserialize;
use std::{any::TypeId, collections::HashMap, fmt::Debug};
use crate::riker::system::Run;
use riker_patterns::ask::ask;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DidCommActorMsg {
  Trustping(Trustping),
  DidRequest(DidRequest),
}

impl From<Trustping> for DidCommActorMsg {
  fn from(other: Trustping) -> Self {
    DidCommActorMsg::Trustping(other)
  }
}
impl From<DidRequest> for DidCommActorMsg {
  fn from(other: DidRequest) -> Self {
    DidCommActorMsg::DidRequest(other)
  }
}
// Communicator trait specifies all default request->response mappings
// The drawback is that we can't refer to fields
// TODO: add account (adapter trait)
pub trait DidCommunicator {
  type Msg: 'static + Clone + Debug + Send;

  fn receive_trustping(&mut self, _ctx: &Context<Self::Msg>, _msg: Trustping, sender: Sender) {
    println!("trustping received");
    sender
      .expect("Sender should exists")
      // TODO get rid of the Response enum?!
      .try_tell(DidCommActorResponse::Trustping(TrustpingResponse::default()), None)
      .expect("Sender should receive the response");
  }

  fn receive_did_request(&mut self, _ctx: &Context<Self::Msg>, _msg: DidRequest, sender: Sender) {
    println!("didrequest received");
    sender
      .expect("Sender should exists")
      .try_tell(
        DidCommActorResponse::DidComm(DidResponse::new("did:example:123".parse().unwrap())),
        None,
      )
      .expect("Sender should receive the response");
  }
}

// Apparently we need to use dynamic dispatch to get around a generic DidCommActor<T: DidCommunicator>. The workaround seems to be needed, since the #actor macro does
// not seem to respect a generic type parameter. Not clear if this really is the case, did https://github.com/riker-rs/riker/pull/124 solve another issue?
pub struct DidCommActor {
  actor: Box<dyn DidCommunicator<Msg = DidCommActorMsg> + Send>,
}

impl DidCommActor {
  fn new<T: 'static + DidCommunicator<Msg = DidCommActorMsg> + Send>(actor: T) -> Self {
    Self { actor: Box::new(actor) }
  }
}

impl Default for DidCommActor {
  fn default() -> Self {
    Self::new(DefaultCommunicator)
  }
}

impl Receive<Trustping> for DidCommActor {
  type Msg = DidCommActorMsg;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: Trustping, sender: Sender) {
    self.actor.receive_trustping(ctx, msg, sender);
  }
}

impl Receive<DidRequest> for DidCommActor {
  type Msg = DidCommActorMsg;

  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: DidRequest, sender: Sender) {
    self.actor.receive_did_request(ctx, msg, sender)
  }
}

impl Actor for DidCommActor {
  // we used the #[actor] attribute so DidCommActorMsg is the Msg type
  type Msg = DidCommActorMsg;

  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // Use the respective Receive<T> implementation
    match msg {
      DidCommActorMsg::Trustping(msg) => Receive::<Trustping>::receive(self, ctx, msg, sender),
      DidCommActorMsg::DidRequest(msg) => Receive::<DidRequest>::receive(self, ctx, msg, sender),
    }
  }
}

pub struct EncryptedDidCommActor {
  // this prevents from wrapping signed envelopes in encrypted envelopes
  inner: ActorRef<DidCommActorMsg>,
  recipients: PublicKey,
  keypair: KeyPair,
  algorithm: EncryptionAlgorithm,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DidCommActorResponse {
  Trustping(TrustpingResponse),
  DidComm(DidResponse),
}

impl Actor for EncryptedDidCommActor {
  type Msg = envelope::Encrypted;
  fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
    // TODO async tasks vs channel?
    let msg = msg
      .unpack::<DidCommActorMsg>(self.algorithm, &self.keypair.secret(), &self.recipients)
      .unwrap();
    let handle: RemoteHandle<DidCommActorResponse> =
      ask::<DidCommActorMsg, _, _, _>(&ctx.system.clone(), &self.inner, msg);
    ctx
      .run(handle)
      .map(|fut| {
        let did_comm_msg = async_std::task::block_on(async { fut.await });
        let response_envelope =
          Encrypted::pack(&did_comm_msg, self.algorithm, &[self.recipients.clone()], &self.keypair).unwrap();

        sender
          .expect("sender must be present")
          .try_tell(response_envelope, ctx.myself())
          .expect("could not send");
      })
      .expect("could not run");
  }
}

impl ActorFactoryArgs<(ActorRef<DidCommActorMsg>, PublicKey, KeyPair, EncryptionAlgorithm)> for EncryptedDidCommActor {
  fn create_args(config: (ActorRef<DidCommActorMsg>, PublicKey, KeyPair, EncryptionAlgorithm)) -> Self {
    Self {
      inner: config.0,
      recipients: config.1,
      keypair: config.2,
      algorithm: config.3,
    }
  }
}
pub struct DefaultCommunicator;
impl DidCommunicator for DefaultCommunicator {
  type Msg = DidCommActorMsg;
}

// !  overwriting handlers requires a tiny bit of boilerplate, creating the Actor should be as easy as DidCommActor { actor: MyCommunicator }

/// Custom communicator that overwrites receive_trustping
pub struct MyCommunicator;
impl DidCommunicator for MyCommunicator {
  type Msg = DidCommActorMsg;

  fn receive_trustping(&mut self, _ctx: &Context<Self::Msg>, _msg: Trustping, sender: Sender) {
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



/// Dynamic request handler for Actors with Enum Messages.
/// Allows to 
struct RequestHandler<'a, A: Actor, M> {
  mapping: HashMap<TypeId, Box<dyn Fn(&'a mut A, &'a Context<A::Msg>, M, Sender)>>
}

type HandlerFn<'a, A: Actor, M> = dyn Fn(&'a mut A, &'a Context<A::Msg>, M, Sender);

impl <'a, A: Actor, M: 'static + Message> RequestHandler<'a, A, M> {
  fn register(&'a mut self, callback: Box<HandlerFn<'a, A, M>>) -> Option<Box<HandlerFn<'a, A, M>>> {
    self.mapping.insert(TypeId::of::<M>(), callback)
  }
  fn get(&'a self, key: &TypeId) -> Option<&Box<HandlerFn<'a, A, M>>> {
    self.mapping.get(key)
  }
  fn handle(&'a self, actor: &'a mut A, ctx: &'a Context<A::Msg>, msg: M, sender: Sender) -> Result<(), String>{
  
    let callback = self.get(&TypeId::of::<M>()).ok_or_else(|| format!("callback not found for  {}", std::any::type_name::<M>()))?;
    callback(actor, ctx, msg, sender);
    Ok(())
  }
}

trait Handler<'a, A: Actor> {
  type Request;
  type Response;

  fn handle(&'a self, actor: &'a mut A, ctx: &'a Context<A::Msg>, msg: Self::Request, sender: Sender) -> Result<Self::Response, String>;
}

struct TrustPingHandler;
impl <'a> Handler<'a, DidCommActor> for TrustPingHandler {
  type Request = Trustping;
  type Response = TrustpingResponse;

  fn handle(&'a self, actor: &'a mut DidCommActor, ctx: &'a Context<<DidCommActor as Actor>::Msg>, msg: Self::Request, sender: Sender) -> Result<Self::Response, String> {
      dbg!("trustping received");
      Ok(TrustpingResponse::default())
  }
}