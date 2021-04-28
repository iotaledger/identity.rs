// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_std::task;
use identity_account::{account::Account, storage::MemStore};
use identity_comm::actor::Request;
use identity_comm::actor::Response;
use identity_comm::actor::SignedActor;
use identity_comm::message::Message;
use identity_comm::message::Trustping;
use identity_comm::message::TrustpingResponse;
use identity_comm::{
  actor::DidCommActor,
  envelope::{SignatureAlgorithm, Signed},
};
use identity_comm::{
  actor::EncryptedActor,
  message::{DidRequest, DidResponse},
};
use identity_comm::{
  envelope::{Encrypted, EncryptionAlgorithm},
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

#[tokio::main]
async fn main() -> Result<(), String> {
  let keypair_sig = KeyPair::new_ed25519().unwrap();
  let keypair_enc = ed25519_to_x25519_keypair(keypair_sig.clone()).unwrap();

  let algo = EncryptionAlgorithm::A256GCM;
  // set up the actor system
  let sys = ActorSystem::new().unwrap();

  // create test account
  let account = Account::new(MemStore::default()).await.unwrap();
  account.create(Default::default()).await.unwrap(); // create new chain
  let doc = {
    let chain = account.try_with_index(|index| index.try_first()).unwrap();
    account.get(chain).await
  }
  .unwrap();
  // create instance of DidCommActor actor
  let actor = sys
    .actor_of_args::<DidCommActor<MemStore>, Arc<Account<MemStore>>>(
      "didcomm_actor",
      Arc::new(account),
    )
    .unwrap();

  // ask the actor for trustping
  let msg = Trustping::new(Url::parse("http://bobsworld.com").unwrap());
  let r: Response = task::block_on(ask(&sys, &actor, msg.clone()));

  assert_eq!(
    format!("{:?}", r),
    format!("{:?}", Response::Trustping(TrustpingResponse::default()))
  );

  // ask the actor for did
  let did_msg = DidRequest::new(Url::parse("http://bobsworld.com").unwrap());
  let r: Response = task::block_on(ask(&sys, &actor, did_msg.clone()));

  assert_eq!(
    format!("{:?}", r),
    format!("{:?}", Response::Did(DidResponse::new(doc.id().clone())))
  );

  /* -------------------------------------------------------------------------- */
  /* ENCRYPTED */
  /* -------------------------------------------------------------------------- */

  // send another trustping, this time in an encrypted envelope
  let encrypting_actor = sys
    .actor_of_args::<EncryptedActor<Request, Response>, (ActorRef<Request>, PublicKey, KeyPair, EncryptionAlgorithm)>(
      "did_comm_enc_actor",
      (actor, keypair_enc.public().clone(), keypair_enc.clone(), algo),
    )
    .unwrap();

  let encrypted_msg = Request::Trustping(msg)
    .pack_auth(algo, &[keypair_enc.public().clone()], &keypair_enc)
    .unwrap();
  // send encrypted msg to encrypted actor
  let r_encrypted: Encrypted = task::block_on(ask(&sys, &encrypting_actor, encrypted_msg.clone()));
  let res: Response = r_encrypted
    .unpack::<Response>(algo, keypair_enc.secret(), keypair_enc.public())
    .unwrap();

  assert_eq!(
    format!("{:?}", res),
    format!("{:?}", Response::Trustping(TrustpingResponse::default()))
  );

  /* -------------------------------------------------------------------------- */
  /* Signed<Encrypted> */
  /* -------------------------------------------------------------------------- */

  let algo_sig = SignatureAlgorithm::EdDSA;
  let signing_actor = sys
    .actor_of_args::<SignedActor<Encrypted, Encrypted>, (ActorRef<Encrypted>, PublicKey, KeyPair, SignatureAlgorithm)>(
      "did_comm_sig_actor",
      (
        encrypting_actor,
        keypair_sig.public().clone(),
        keypair_sig.clone(),
        algo_sig,
      ),
    )
    .unwrap();

  let signed_msg = encrypted_msg.pack_non_repudiable(algo_sig, &keypair_sig).unwrap();
  // send signed msg to signed actor
  let r_signed: Signed = task::block_on(ask(&sys, &signing_actor, signed_msg.clone()));
  // The signed response wraps an encrypted response which wraps the actual response
  let res: Response = r_signed
    .unpack::<Encrypted>(algo_sig, keypair_sig.public())
    .unwrap()
    .unpack::<Response>(algo, keypair_enc.secret(), keypair_enc.public())
    .unwrap();

  assert_eq!(
    format!("{:?}", res),
    format!("{:?}", Response::Trustping(TrustpingResponse::default()))
  );

  Ok(())
}
