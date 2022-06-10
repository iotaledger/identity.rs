# Iota Identity: Credentials 

This crate contains types representing *credentials* and *presentations* defined in the [W3C VC-Data model]((https://www.w3.org/TR/vc-data-model/). An overview of these concepts and how to work with them in the context of the Iota Identity Framework can be found [in the wiki](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview). 


# Example
Constructing a `Credential` using the `CredentialBuilder`. 

```rust 
  use identity_credential::credential::{Credential, CredentialBuilder, Subject, Issuer};
  use serde_json::json;
  use serde_json::Value;

  // Construct a `Subject` from json
  let json_subject: Value = json!({
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  });
  let subject: Subject = serde_json::from_value(json_subject).unwrap();

  // Construct an `Issuer` from json
  let json_issuer: Value = json!({
    "id": "did:example:76e12ec712ebc6f1c221ebfeb1f",
    "name": "Example University"
  });

  let issuer: Issuer = serde_json::from_value(json_issuer).unwrap();

  let credential: Credential = CredentialBuilder::default()
    .context(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap())
    .id(Url::parse("http://example.edu/credentials/3732").unwrap())
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .issuer(issuer)
    .issuance_date(Timestamp::parse("2010-01-01T00:00:00Z").unwrap())
    .build()
    .unwrap();

```
# Important 
Although the `CredentialBuilder` generates a `Credential` it cannot be considered a *verifiable credential* until it has been signed by the issuer. See [this example](https://github.com/iotaledger/identity.rs/blob/support/v0.5/examples/account/create_vc.rs) for a full example explaining how to create a credential with the corresponding issuer's signature. 


