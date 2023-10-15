use super::{Disclosure, Hasher, ShaHasher};
use serde_json::{json, Map, Value};

///
pub struct SdEncoder<H = ShaHasher>
where
  H: Hasher,
{
  ///
  pub json: Map<String, Value>,
  hasher: H,
}

impl<H> SdEncoder<H>
where
  H: Hasher,
{
  ///
  pub fn new(hasher: H, json: &str) -> Self {
    Self {
      json: serde_json::from_str(json).unwrap(),
      hasher,
    }
  }

  ///
  pub fn conceal(&mut self, path: &[&str], salt: Option<String>) -> Disclosure {
    if path.len() == 0 {
      todo!();
    }

    let mut parent_value = &mut self.json;
    let mut target_property = path[0];

    for index in 1..path.len() {
      match parent_value.get(target_property).unwrap() {
        Value::Object(_) => {
          parent_value = parent_value.get_mut(path[index - 1]).unwrap().as_object_mut().unwrap();
          target_property = path[index];
        }
        _ => panic!(),
      }
    }

    let disclosure = Disclosure::new(
      salt,
      Some(target_property.to_owned()),
      parent_value.remove(target_property).unwrap().to_string().to_owned(),
    );

    let hash = self.hasher.digest(disclosure.as_str());

    match parent_value.get_mut("_sd") {
      Some(sd_value) => {
        match sd_value {
          Value::Array(value) => value.push(Value::String(hash)),
          _ => todo!(), //error ?
        }
      }
      None => {
        parent_value.insert("_sd".to_owned(), Value::Array(vec![Value::String(hash)]));
      }
    }
    disclosure
  }

  ///
  pub fn conceal_array_entry(&mut self, path: &[&str], element_index: usize, salt: Option<String>) -> Disclosure {
    if path.len() == 0 {
      todo!();
    }

    let mut parent_value = &mut self.json;
    let mut target_property = path[0];

    for index in 1..path.len() {
      match parent_value.get(target_property).unwrap() {
        Value::Object(_) => {
          parent_value = parent_value.get_mut(path[index - 1]).unwrap().as_object_mut().unwrap();
          target_property = path[index];
        }
        _ => panic!(),
      }
    }

    let array = parent_value.get_mut(target_property).unwrap().as_array_mut().unwrap();
    //todo check array length.

    if let Some(element_value) = array.get_mut(element_index) {
      let disclosure = Disclosure::new(salt, None, element_value.to_string());
      let hash = self.hasher.digest(disclosure.as_str());
      let tripledot = json!({"...": hash});
      *element_value = tripledot;
      disclosure
    } else {
      panic!("element doesn't exist at this index");
    }
  }

  ///
  pub fn add_decoy(path: &[&str], value: Option<String>) {
    todo!();
  }

  ///
  pub fn shuffle(path: &[&str]) {
    todo!();
  }
}

#[cfg(test)]
mod test {
  use super::SdEncoder;
  use crate::sd_jwt::ShaHasher;
  use serde_json::{json, Value};

  #[test]
  fn test() {
    let json = json!({
      "id": "blabla",
      "something": {
        "abc": true
      },
    });
    let hasher = ShaHasher::new();
    let stringi = json.to_string();
    let mut encoder = SdEncoder::new(hasher, &stringi);
    encoder.conceal(&["something", "abc"], Some("salttt".to_owned()));
  }

  #[test]
  fn test_vc() {
    // Test values partially taken from
    // https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#name-example-1-sd-jwt
    let json = json!({
      "sub": "user_42",
      "given_name": "John",
      "family_name": "Doe",
      "email": "johndoe@example.com",
      "phone_number": "+1-202-555-0101",
      "phone_number_verified": true,
      "address": {
        "street_address": "123 Main St",
        "locality": "Anytown",
        "region": "Anystate",
        "country": "US"
      },
      "birthdate": "1940-01-01",
      "updated_at": 1570000000,
      "nationalities": [
        "US",
        "DE"
      ]
    });
    let hasher = ShaHasher::new();
    let stringi = json.to_string();
    let mut encoder = SdEncoder::new(hasher, &stringi);
    let disclosure = encoder.conceal(&["given_name"], Some("2GLC42sKQveCfGfryNRN9w".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["family_name"], Some("eluV5Og3gSNII8EYnsxA_A".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["email"], Some("6Ij7tM-a5iVPGboS5tmvVA".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["phone_number"], Some("eI8ZWm9QnKPpNPeNenHdhQ".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["phone_number_verified"], Some("Qg_O64zqAxe412a108iroA".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["address"], Some("AJx-095VPrpTtN4QMOqROA".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["birthdate"], Some("Pc33JM2LchcU_lHggv_ufQ".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal(&["updated_at"], Some("G02NSrQfjFXQ7Io09syajA".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal_array_entry(&["nationalities"], 0, Some("lklxF5jMYlGTPUovMNIvCA".to_owned()));
    println!("{}", disclosure.to_string());

    let disclosure = encoder.conceal_array_entry(&["nationalities"], 1, Some("nPuoQnkRFq3BIeAm7AnXFA".to_owned()));
    println!("{}", disclosure.to_string());

    let expected = json!({
      "_sd": [
        "jsu9yVulwQQlhFlM_3JlzMaSFzglhQG0DpfayQwLUK4",
        "TGf4oLbgwd5JQaHyKVQZU9UdGE0w5rtDsrZzfUaomLo",
        "JzYjH4svliH0R3PyEMfeZu6Jt69u5qehZo7F7EPYlSE",
        "PorFbpKuVu6xymJagvkFsFXAbRoc2JGlAUA2BA4o7cI",
        "XQ_3kPKt1XyX7KANkqVR6yZ2Va5NrPIvPYbyMvRKBMM",
        "A9gUc163WWnsyoMy8a3AVnJLBw-CZbttJTnsgxkahyo",
        "gbOsI4Edq2x2Kw-w5wPEzakob9hV1cRD0ATN3oQL9JM",
        "CrQe7S5kqBAHt-nMYXgc6bdt2SH5aTY1sU_M-PgkjPI"
      ],
      "nationalities": [
        {
          "...": "pFndjkZ_VCzmyTa6UjlZo3dh-ko8aIKQc9DlGzhaVYo"
        },
        {
          "...": "7Cf6JkPudry3lcbwHgeZ8khAv1U1OSlerP0VkBJrWZ0"
        }
      ],
      "sub": "user_42"
    });

    assert_eq!(Value::Object(encoder.json), expected);
  }

  #[test]
  fn test_nested() {
    // todo https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#name-nested-data-in-sd-jwts
  }

  #[test]
  fn test_recursive_disclosures() {
    // todo https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#name-option-3-sd-jwt-with-recurs
  }
}
