extern crate riker;
use riker::actors::*;

// Define the messages we'll use
#[derive(Clone, Debug)]
pub struct Add;

#[derive(Clone, Debug)]
pub struct Sub;

#[derive(Clone, Debug)]
pub struct Print;

// Define the Actor and use the 'actor' attribute
// to specify which messages it will receive
#[actor(Add, Sub, Print)]
pub struct IdentityActor {
    count: u32,
}

impl ActorFactoryArgs<u32> for IdentityActor {
    fn create_args(count: u32) -> Self {
        Self { count }
    }
}

impl Actor for IdentityActor {
    type Msg = IdentityActorMsg;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        // Use the respective Receive<T> implementation
        self.receive(ctx, msg, sender);
    }
}

impl Receive<Add> for IdentityActor {
    type Msg = IdentityActorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Add, _sender: Sender) {
        self.count += 1;
    }
}

impl Receive<Sub> for IdentityActor {
    type Msg = IdentityActorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Sub, _sender: Sender) {
        self.count -= 1;
    }
}

impl Receive<Print> for IdentityActor {
    type Msg = IdentityActorMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Print, _sender: Sender) {
        println!("Total counter value: {}", self.count);
    }
}