extern crate rmp_serde as rmps;
use crate::did::IotaDocument;
use identity_core::crypto::KeyPair;
use rmps::{Deserializer, Serializer};
use serde::Serialize;

#[test]
fn test_cbor() {
  let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
  let mut document: IotaDocument = IotaDocument::new(&keypair).unwrap();
  document.sign(&keypair.private()).unwrap();
  let mut buf = Vec::new();

  document.serialize(&mut Serializer::new(&mut buf));

  println!("{}", buf.len());
  println!("{}", document.to_string().as_bytes().len());
}
