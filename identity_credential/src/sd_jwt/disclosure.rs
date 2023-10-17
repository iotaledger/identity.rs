use crypto::utils::rand::fill;
use identity_verification::jwu::{decode_b64_json, encode_b64};
use rand::distributions::DistString;
use serde_json::Value;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disclosure {
  ///
  pub salt: String,
  ///
  pub claim_name: Option<String>,
  ///
  pub claim_value: Value,
  ///
  pub disclosure: String,
}

impl Disclosure {
  ///
  pub fn new(salt: Option<String>, claim_name: Option<String>, claim_value: impl Into<Value>) -> Self {
    let claim_value: Value = claim_value.into();
    let salt = salt.unwrap_or(Self::gen_rand());
    let result = match claim_name {
      Some(name) => {
        let input: String = format!("[\"{}\", \"{}\", {}]", &salt, &name, &claim_value.to_string());
        let encoded = encode_b64(&input);
        Self {
          salt,
          claim_name: Some(name),
          claim_value,
          disclosure: encoded,
        }
      }
      None => {
        let input: String = format!("[\"{}\", {}]", &salt, &claim_value.to_string());
        let encoded = encode_b64(&input);
        Self {
          salt,
          claim_name: None,
          claim_value,
          disclosure: encoded,
        }
      }
    };

    result
  }

  ///
  pub fn parse(disclosure: String) -> Self {
    let decoded: Vec<Value> = decode_b64_json(&disclosure).unwrap();
    if decoded.len() == 2 {
      Self {
        salt: decoded.get(0).unwrap().as_str().unwrap().to_owned(),
        claim_name: None,
        claim_value: decoded.get(1).unwrap().clone(),
        disclosure,
      }
    } else if decoded.len() == 3 {
      Self {
        salt: decoded.get(0).unwrap().as_str().unwrap().to_owned(),
        claim_name: Some(decoded.get(1).unwrap().as_str().unwrap().to_owned()),
        claim_value: decoded.get(2).unwrap().clone(),
        disclosure,
      }
    } else {
      panic!("");
    }
  }

  ///
  pub fn to_string(&self) -> String {
    self.disclosure.clone()
  }

  ///
  pub fn as_str(&self) -> &str {
    &self.disclosure
  }

  ///
  pub fn into_string(self) -> String {
    self.disclosure
  }

  fn gen_rand() -> String {
    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 128)
  }
}

#[cfg(test)]
mod test {
  use crypto::utils::rand::{fill, gen};
  use rand::distributions::DistString;

  use super::Disclosure;
  use crate::sd_jwt::{Hasher, ShaHasher};

  // Test values from:
  // https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#appendix-A.2-7
  #[test]
  fn test() {
    // Test creating.
    let disclosure = Disclosure::new(
      Some("2GLC42sKQveCfGfryNRN9w".to_owned()),
      Some("time".to_owned()),
      "2012-04-23T18:25Z".to_owned(),
    );
    assert_eq!(
      "WyIyR0xDNDJzS1F2ZUNmR2ZyeU5STjl3IiwgInRpbWUiLCAiMjAxMi0wNC0yM1QxODoyNVoiXQ".to_owned(),
      disclosure.to_string()
    );

    // Test the SHA hash.
    let hasher = ShaHasher::new();
    let hash = hasher.digest(disclosure.as_str());
    assert_eq!("vTwe3raHIFYgFA3xaUD2aMxFz5oDo8iBu05qKlOg9Lw", hash);

    // Test parsing.
    let parsed =
      Disclosure::parse("WyIyR0xDNDJzS1F2ZUNmR2ZyeU5STjl3IiwgInRpbWUiLCAiMjAxMi0wNC0yM1QxODoyNVoiXQ".to_owned());
    assert_eq!(parsed, disclosure);
  }

  // Test values from:
  // https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#appendix-A.2-7
  #[test]
  fn test_2() {
    // Test creating.
    let disclosure = Disclosure::new(
      Some("eluV5Og3gSNII8EYnsxA_A".to_owned()),
      Some("given_name".to_owned()),
      "太郎".to_owned(),
    );
    assert_eq!(
      "WyJlbHVWNU9nM2dTTklJOEVZbnN4QV9BIiwgImdpdmVuX25hbWUiLCAiXHU1OTJhXHU5MGNlIl0".to_owned(),
      disclosure.to_string()
    );
  }

  #[test]
  fn test_3() {
    // let mut salt = [0_u8; 128];
    // fill(&mut salt).unwrap();
    // println!("{:?}", &salt);
    // let s = String::from_str(&salt);
    let s = rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 128);
    println!("{}", s);

    // println!("{:?}", "hello".to_owned().as_bytes());
  }
}
