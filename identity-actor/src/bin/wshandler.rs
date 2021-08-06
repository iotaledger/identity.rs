use p2p::{InitKeypair, Keypair};
use libp2p::{tcp::TcpConfig, websocket::WsConfig, Multiaddr};

use identity_actor::{actor_builder::ActorBuilder, asyncfn::AsyncFn, errors::Result, StorageHandler};

#[tokio::main]
async fn main() -> Result<()> {
  // pretty_env_logger::init();

  let id_keys = Keypair::generate_ed25519();
  let tcp_transport = TcpConfig::new().nodelay(true);
  let transport = WsConfig::new(tcp_transport);

  let addr: Multiaddr = "/ip4/0.0.0.0/tcp/12345/ws".parse().unwrap();

  let mut comm = ActorBuilder::new()
    .keys(InitKeypair::IdKeys(id_keys))
    .listen_on(addr.clone())
    .build_with_transport(transport)
    .await?;

  let handler = StorageHandler::new().await.unwrap();

  comm.add_handler(handler)
      .add_method("storage/list", AsyncFn::new(StorageHandler::list));

  let peer_id = comm.peer_id();

  println!("Listening on {:?} with PeerId: {}", addr, peer_id.to_base58());

  // Blocks forever
  comm.join().await;

  Ok(())
}
