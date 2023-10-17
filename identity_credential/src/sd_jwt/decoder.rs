use std::collections::BTreeMap;

use serde_json::{Map, Value};

use super::{Disclosure, Hasher, ShaHasher};

///
pub struct SdPayloadDecoder<H = ShaHasher>
where
  H: Hasher,
{
  ///
  hasher: H,
}

impl SdPayloadDecoder {
  ///
  pub fn new() -> Self {
    Self {
      hasher: ShaHasher::new(),
    }
  }
}

impl<H> SdPayloadDecoder<H>
where
  H: Hasher,
{
  ///
  pub fn with_custom_hasher(hasher: H) -> Self {
    Self { hasher }
  }

  ///
  pub fn decode(&self, payload: &Map<String, Value>, disclosures: &Vec<String>) -> Map<String, Value> {
    let disclosures: BTreeMap<String, Disclosure> = disclosures
      .iter()
      .map(|disclosure| {
        (
          self.hasher.digest(disclosure),
          Disclosure::parse(disclosure.to_string()),
        )
      })
      .collect();

    self.decode_object(payload, &disclosures)
  }

  ///
  /// todo:  If any digests were found more than once in the previous step, the SD-JWT MUST be rejected.
  pub fn decode_object(
    &self,
    payload: &Map<String, Value>,
    disclosures: &BTreeMap<String, Disclosure>,
  ) -> Map<String, Value> {
    let mut output: Map<String, Value> = payload.clone();
    for (key, value) in payload.iter() {
      if key == "_sd" {
        let sd_array: &Vec<Value> = value.as_array().unwrap();
        for sd_entry in sd_array {
          let hash_in_sd = sd_entry.as_str().unwrap().to_string();
          if let Some(disclosure) = disclosures.get(&hash_in_sd) {
            // if output.contains_key(&disclosure.claim_name.expect("claim name must be present for object")) {
            //   panic!("claim already exist");
            // }

            output.insert(
              disclosure.claim_name.clone().unwrap(),
              disclosure.claim_value.clone(), // Value::from_str(&disclosure.claim_value).unwrap(),
            );
          }
        }
        output.remove("_sd");
        continue;
      }

      match value {
        Value::Object(object) => {
          let decoded_object = self.decode_object(object, disclosures);
          if !decoded_object.is_empty() {
            output.insert(key.to_string(), Value::Object(decoded_object));
          }
        }
        Value::Array(array) => {
          let decoded_array: Vec<Value> = self.decode_array(array, disclosures);
          if !decoded_array.is_empty() {
            output.insert(key.to_string(), Value::Array(decoded_array));
          }
        }
        _ => {}
      }
    }
    // todo: check for double concealed values.
    output
  }

  ///
  pub fn decode_array(&self, array: &Vec<Value>, disclosures: &BTreeMap<String, Disclosure>) -> Vec<Value> {
    let mut output: Vec<Value> = vec![];

    for value in array.iter() {
      if let Some(object) = value.as_object() {
        for (key, value) in object.iter() {
          if key == "..." {
            if object.keys().len() != 1 {
              panic!("SD object with multiple `...` keys")
            }
            let hash_in_array = value.as_str().unwrap().to_string();
            if let Some(disclosure) = disclosures.get(&hash_in_array) {
              if disclosure.claim_name.is_some() {
                panic!("array length must be 2");
              }
              output.push(disclosure.claim_value.clone());
            }
          } else {
            output.push(Value::Object(self.decode_object(object, disclosures)));
          }
        }
      } else {
        output.push(value.clone());
      }
    }

    output
  }
}
#[cfg(test)]
mod test {
  use serde_json::{json, Value};

  use crate::sd_jwt::ShaHasher;

  use super::SdPayloadDecoder;

  #[test]
  fn test_vc() {
    // Test values partially taken from
    // https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-05.html#name-example-1-sd-jwt
    let json = json!({
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

    let mut decoder = SdPayloadDecoder::new();
    let disclosures = vec![
      "WyIyR0xDNDJzS1F2ZUNmR2ZyeU5STjl3IiwgImdpdmVuX25hbWUiLCAiSm9obiJd",
      "WyJsa2x4RjVqTVlsR1RQVW92TU5JdkNBIiwgIlVTIl0",
      "WyIyR0xDNDJzS1F2ZUNmR2ZyeU5STjl3IiwgImdpdmVuX25hbWUiLCAiSm9obiJd",
      "WyJlbHVWNU9nM2dTTklJOEVZbnN4QV9BIiwgImZhbWlseV9uYW1lIiwgIkRvZSJd",
      "WyI2SWo3dE0tYTVpVlBHYm9TNXRtdlZBIiwgImVtYWlsIiwgImpvaG5kb2VAZXhhbXBsZS5jb20iXQ",
      "WyJlSThaV205UW5LUHBOUGVOZW5IZGhRIiwgInBob25lX251bWJlciIsICIrMS0yMDItNTU1LTAxMDEiXQ",
      "WyJRZ19PNjR6cUF4ZTQxMmExMDhpcm9BIiwgInBob25lX251bWJlcl92ZXJpZmllZCIsIHRydWVd",
      "WyJBSngtMDk1VlBycFR0TjRRTU9xUk9BIiwgImFkZHJlc3MiLCB7ImNvdW50cnkiOiJVUyIsImxvY2FsaXR5IjoiQW55dG93biIsInJlZ2lvbiI6IkFueXN0YXRlIiwic3RyZWV0X2FkZHJlc3MiOiIxMjMgTWFpbiBTdCJ9XQ",
      "WyJQYzMzSk0yTGNoY1VfbEhnZ3ZfdWZRIiwgImJpcnRoZGF0ZSIsICIxOTQwLTAxLTAxIl0",
      "WyJHMDJOU3JRZmpGWFE3SW8wOXN5YWpBIiwgInVwZGF0ZWRfYXQiLCAxNTcwMDAwMDAwXQ",
      "WyJsa2x4RjVqTVlsR1RQVW92TU5JdkNBIiwgIlVTIl0",
      "WyJuUHVvUW5rUkZxM0JJZUFtN0FuWEZBIiwgIkRFIl0"
    ];
    // let decoded = decoder.decode(&json.as_object().unwrap(), &disclosures);
    // println!(
    //   ">>>>>>>>>>> \n {} >>>>>>>>>> \n {:?}",
    //   serde_json::to_string_pretty(&Value::Object(decoded.clone())).unwrap(),
    //   decoded
    // );
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
