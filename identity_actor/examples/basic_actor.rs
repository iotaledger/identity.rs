extern crate riker;
use riker::actors::*;
use std::time::Duration;

use identity_actor::actor::{IdentityActor, Add, Sub, Print};


// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let actor = sys.actor_of_args::<IdentityActor, _>("counter", 0).unwrap();
    actor.tell(Add, None);
    actor.tell(Print, None);
    actor.tell(Add, None);
    actor.tell(Print, None);
    actor.tell(Sub, None);
    actor.tell(Print, None);
    sys.print_tree();
    // force main to wait before exiting program
    std::thread::sleep(Duration::from_millis(500));
}