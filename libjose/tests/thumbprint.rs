use libjose::jwk::Jwk;
use libjose::jwk::SHA256;
use serde::Deserialize;

const JSON: &[u8] = include_bytes!("fixtures/thumbprint/1.json");

#[derive(Deserialize)]
struct Fixture {
  input: Jwk,
  output: String,
}

#[test]
fn test_thumbprint_example() {
  let fixture: Fixture = serde_json::from_slice(JSON).unwrap();
  let thumbprint: String = fixture.input.thumbprint_b64(SHA256).unwrap();

  assert_eq!(thumbprint, fixture.output);
}
