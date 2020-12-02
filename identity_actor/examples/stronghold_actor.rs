//!
//!
//! Run with:
//!
//! ```
//! cargo run --example stronghold_actor
//! ```


extern crate riker;
use riker::actors::*;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use libp2p::core::identity::Keypair;
use stronghold_communication::{
    actor::{CommunicationActor, CommunicationEvent},
    behaviour::message::P2PReqResEvent,
};
use identity_actor::sh_actor::{Request, Response, TestActor};

fn main() {
    let local_keys = Keypair::generate_ed25519();
    let sys = ActorSystem::new().unwrap();
    let chan: ChannelRef<CommunicationEvent<Request, Response>> = channel("remote-peer", &sys).unwrap();
    sys.actor_of_args::<CommunicationActor<Request, Response>, _>("communication-actor", (local_keys, chan.clone()))
        .unwrap();
    sys.actor_of_args::<TestActor, _>("test-actor", chan).unwrap();
    std::thread::sleep(Duration::from_secs(600));
}