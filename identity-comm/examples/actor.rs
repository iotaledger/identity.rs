use async_std::task;
use identity_comm::actor::DidCommActor;
use identity_comm::actor::DidCommActorMsg;
use identity_comm::actor::DidCommActorResponse;
use identity_comm::actor::EncryptedCommunicator;
use identity_comm::message::Message;
use identity_comm::message::Trustping;
use identity_comm::message::TrustpingResponse;
use identity_comm::{
  envelope::{self, Encrypted, EncryptionAlgorithm},
  error::Result,
};
use identity_core::common::Url;
use identity_core::crypto::{KeyPair, PublicKey, SecretKey};
use libjose::utils::ed25519_to_x25519_public;
use libjose::utils::ed25519_to_x25519_secret;
use riker::actor::ActorRef;
use riker::actor::ActorRefFactory;
use riker::actors::ActorSystem;
use riker_patterns::ask::ask;

fn ed25519_to_x25519(keypair: KeyPair) -> Result<(PublicKey, SecretKey)> {
  Ok((
    ed25519_to_x25519_public(keypair.public())?.to_vec().into(),
    ed25519_to_x25519_secret(keypair.secret())?.to_vec().into(),
  ))
}

fn ed25519_to_x25519_keypair(keypair: KeyPair) -> Result<KeyPair> {
  // This is completely wrong but `type_` is never used around here
  let type_ = keypair.type_();
  let (public, secret) = ed25519_to_x25519(keypair)?;
  Ok((type_, public, secret).into())
}

fn main() -> Result<(), String> {
  let keypair = KeyPair::new_ed25519().unwrap();
  let keypair = ed25519_to_x25519_keypair(keypair).unwrap();

  let algo = EncryptionAlgorithm::A256GCM;
  // set up the actor system
  let sys = ActorSystem::new().unwrap();

  // create instance of DidCommActor actor
  let actor = sys.actor_of::<DidCommActor>("did_comm_actor").unwrap();

  // ask the actor
  let msg = Trustping::new(Url::parse("http://bobsworld.com").unwrap());
  let r: DidCommActorResponse = task::block_on(ask(&sys, &actor, msg.clone()));

  assert_eq!(
    format!("{:?}", r),
    format!("{:?}", DidCommActorResponse::Trustping(TrustpingResponse::default()))
  );

  /* -------------------------------------------------------------------------- */
  /*                                  ENCRYPED                                  */
  /* -------------------------------------------------------------------------- */

  // send another trustping, this time in an encrypted envelope
  let encrypted_actor = sys
    .actor_of_args::<EncryptedCommunicator, (ActorRef<DidCommActorMsg>, PublicKey, KeyPair, EncryptionAlgorithm)>(
      "did_comm_enc_actor",
      (actor, keypair.public().clone(), keypair.clone(), algo),
    )
    .unwrap();

  let encrypted_msg = DidCommActorMsg::Trustping(msg)
    .pack_auth(algo, &[keypair.public().clone()], &keypair)
    .unwrap();
  // send encrypted msg to encrypted actor
  let r_encrypted: Encrypted = task::block_on(ask(&sys, &encrypted_actor, encrypted_msg.clone()));
  let res = r_encrypted
    .unpack::<DidCommActorResponse>(algo, keypair.secret(), keypair.public())
    .unwrap();
    
  assert_eq!(
    format!("{:?}", res),
    format!("{:?}", DidCommActorResponse::Trustping(TrustpingResponse::default()))
  );

  Ok(())
}
