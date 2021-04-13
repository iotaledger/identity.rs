use crate::message::{DidRequest, DidResponse, Report, Trustping};
use riker::actor::actor;
use riker::actor::Actor;
use riker::actor::BasicActorRef;
use riker::actor::Context;
use riker::actor::Receive;
use riker::actor::Sender;
use std::fmt::Debug;

pub trait DidCommunicator {
  type Msg: 'static + Clone + Debug + Send;

  fn receive_trustping(&mut self, _ctx: &Context<Self::Msg>, _msg: Trustping, sender: Sender) {
    dbg!("trustping received");
    sender
      .expect("Sender should exists")
      .try_tell(Report::default(), None)
      .expect("Sender should receive the response");
  }

  fn receive_did_request(&mut self, _ctx: &Context<Self::Msg>, _msg: DidRequest, sender: Sender) {
    dbg!("didrequest received");
    sender
      .expect("Sender should exists")
      .try_tell(
        DidResponse::new(
          "did-discovery/1.0/didResponse".to_string(),
          "test-thread".to_string(),
          "did:example:123".parse().unwrap(),
        ),
        None,
      )
      .expect("Sender should receive the response");
  }
}

// Apparently we need to use dynamic dispatch to get around a generic DidCommActor<T: DidCommunicator>. The workaround seems to be needed, since the #actor macro does
// not seem to respect a generic type parameter. Not clear if this really is the case, did https://github.com/riker-rs/riker/pull/124 solve another issue?
#[actor(Trustping, DidRequest)]
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
    self.receive(ctx, msg, sender);
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
      .try_tell(Report::default(), None)
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
impl Receive<Trustping> for MyActor {
  type Msg = MyActorMsg;
  fn receive(&mut self, ctx: &Context<Self::Msg>, msg: Trustping, sender: Sender) {
    dbg!("trustping received - custom conditional response");
    if self.my_state {
      self.receive_trustping(ctx, msg, sender);
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
