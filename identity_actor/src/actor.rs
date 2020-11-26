extern crate riker;
use chronicle_common::actor;
use riker::actors::*;
use tokio::{runtime::Runtime, sync::mpsc::unbounded_channel};

use identity_comm::did_comm::DIDComm;

use crate::message::Message;

use crate::handler::IdentityMessageHandler;
use tokio::sync::mpsc::UnboundedReceiver;

actor!(IdentityBuilder {
  rx: UnboundedReceiver<Message>,
  message_handler: IdentityMessageHandler
});

impl IdentityBuilder {
    /// Builds the Ideneity actor.
    pub fn build(self) -> Identity {
        Identity {
            rx: self.rx.expect("rx is required"),
            message_handler: IdentityMessageHandler::new().expect("failed to initialise account manager"),
        }
    }
}

/// The Account actor.
pub struct Identity {
    rx: UnboundedReceiver<Message>,
    message_handler: IdentityMessageHandler,
}

impl Identity {
    /// Runs the actor.
    pub async fn run(mut self) {
        println!("running identity actor");

        while let Some(message) = self.rx.recv().await {
            self.message_handler.handle(message).await;
        }
    }
}

pub struct IdentityActor {
    identity_message_handler: IdentityMessageHandler,
    runtime: Runtime,
}

impl ActorFactoryArgs<u32> for IdentityActor {
    fn create_args(count: u32) -> Self {
        let actor = Self {
            identity_message_handler: IdentityMessageHandler::new()
                .expect("failed to initialise identity message handler"),
            runtime: Runtime::new().expect("failed to create tokio runtime"),
        };
        actor
    }
}

impl Actor for IdentityActor {
    type Msg = Message;

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        // Use the respective Receive<T> implementation
        // self.receive(ctx, msg, sender);
        let identity_message_handler = &self.identity_message_handler;
        self.runtime.block_on(async move {
            identity_message_handler.handle(msg).await;
        });
    }
}

impl Default for IdentityActor {
    fn default() -> Self {
        let actor = Self {
            identity_message_handler: Default::default(),
            runtime: Runtime::new().expect("failed to create tokio runtime"),
        };
        //   actor.start_polling(POLLING_INTERVAL_MS);
        actor
    }
}

// impl Receive<Add> for IdentityActor {
//     type Msg = IdentityActorMsg;

//     fn receive(&mut self, _ctx: &Context<Self::Msg>, _msg: Add, _sender: Sender) {
//         self.count += 1;
//     }
// }
