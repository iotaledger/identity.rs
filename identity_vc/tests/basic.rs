use identity_vc::prelude::*;
use serde_json::from_str;

fn try_credential(data: &(impl AsRef<str> + ?Sized)) {
  dbg!(from_str::<VerifiableCredential>(data.as_ref()).unwrap());
}

fn try_presentation(data: &(impl AsRef<str> + ?Sized)) {
  dbg!(from_str::<VerifiablePresentation>(data.as_ref()).unwrap());
}

#[test]
fn test_parse_credential() {
  try_credential(include_str!("input/example-01.json"));
  try_credential(include_str!("input/example-02.json"));
  try_credential(include_str!("input/example-03.json"));
  try_credential(include_str!("input/example-04.json"));
  try_credential(include_str!("input/example-05.json"));
  try_credential(include_str!("input/example-06.json"));
  try_credential(include_str!("input/example-07.json"));

  try_credential(include_str!("input/example-09.json"));
  try_credential(include_str!("input/example-10.json"));
  try_credential(include_str!("input/example-11.json"));
  try_credential(include_str!("input/example-12.json"));
  try_credential(include_str!("input/example-13.json"));
}

#[test]
fn test_parse_presentation() {
  try_presentation(include_str!("input/example-08.json"));
}
