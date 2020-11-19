extern crate riker;
use riker::actors::*;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use identity_actor::actor::{IdentityActor, IdentityBuilder};
use identity_actor::message::{Message, MessageType, Response};

// https://github.com/iotaledger/actors.rs/blob/a6e1f79f98e7d994bbb4d4a895b110550ea7f277/wallet/src/lib.rs#L357

// start the system and create an actor
#[tokio::main]
async fn main() -> identity_comm::Result<()> {
    let sys = ActorSystem::new().unwrap();

    // let actor = sys.actor_of_args::<IdentityActor, _>("counter", 0).unwrap();

    let (tx, rx) = unbounded_channel();
    let actor = IdentityBuilder::new().rx(rx).build();
    tokio::spawn(actor.run());

    send_message(&tx, MessageType::TrustPing).await;

    sys.print_tree();
    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(500));
    Ok(())
}

async fn send_message(tx: &UnboundedSender<Message>, message_type: MessageType) -> Response {
    let (message_tx, mut message_rx) = unbounded_channel();
    let message = Message::new(0, message_type, message_tx);
    tx.send(message).unwrap();
    message_rx.recv().await.unwrap()
}
