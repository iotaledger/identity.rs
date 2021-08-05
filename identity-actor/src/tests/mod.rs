mod actor;
mod storage;

use libp2p::{tcp::TcpConfig, Multiaddr, PeerId};
use p2p::{InitKeypair, Keypair};

use crate::{actor_builder::ActorBuilder, Actor};

async fn default_listening_actor() -> (Actor, Multiaddr, PeerId) {
  let id_keys = Keypair::generate_ed25519();
  let transport = TcpConfig::new().nodelay(true);

  let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();

  let mut listening_actor = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .listen_on(addr.clone())
    .build_with_transport(transport)
    .await
    .unwrap();

  let addr = listening_actor.addrs().await.pop().unwrap();
  let peer_id = listening_actor.peer_id();

  (listening_actor, addr, peer_id)
}

async fn default_sending_actor() -> Actor {
  ActorBuilder::new().build().await.unwrap()
}
