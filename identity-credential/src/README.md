Iota Identity Credentials 
=== 

This crate contains types representing verifiable credentials and verifiable presentations as defined in the [W3C Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/). 

The [IOTA Identity Framework Wiki](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview) provides an overview of verifiable credentials and demonstrates how they may be constructed using the building blocks from this crate. 

## Construction
This crate follows the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) for the creation of [`Credentials`](crate::credential::Credential) and [`Presentations`](crate::presentation::Presentation). 

### Example - Credential
Constructing a [`Credential`](crate::credential::Credential) using the [`CredentialBuilder`](crate::credential::CredentialBuilder). 

```rust 
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_credential::credential::Issuer;
use identity_core::common::Url;
use identity_core::common::Timestamp;
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
#### Important 
Although the `CredentialBuilder` generates a `Credential` it cannot be considered a *verifiable credential* until it has been signed by the issuer. See [this example](https://github.com/iotaledger/identity.rs/blob/support/v0.5/examples/account/create_vc.rs) for a full example explaining how to create a credential with the corresponding issuer's signature. 

### Example 
Constructing a [`Presentation`](crate::presentation::Presentation) using the [`PresentationBuilder`](crate::presentation::PresentationBuilder). 

```rust
use identity_credential::credential::Credential; 
use identity_credential::presentation::Presentation; 
use identity_credential::presentation::PresentationBuilder; 
use identity_core::common::Url;

// Build a presentation for the given holder and iterator of credentials 
fn build_presentation(credentials: impl Iterator<Item = Credential>, holder: Url) -> Presentation {
  let presentation_builder: PresentationBuilder = PresentationBuilder::default();
  credentials
    .fold(
      presentation_builder,
      |builder: PresentationBuilder, credential: Credential| builder.credential(credential),
    )
    .holder(holder)
    .build()
    .unwrap()
}
```
#### Important 
A `Presentation` constructed from a `PresentationBuilder` is not automatically a *verifiable presentation*. In order to obtain a verifiable presentation the holder must sign the presentation. All `Credential`s contained in the presentation must also be signed by their respective issuers. See [this example](https://github.com/iotaledger/identity.rs/blob/support/v0.5/examples/account/create_vp.rs) for a full example explaining how to create a verifiable presentation. 

## Credentials and Presentations from JSON 
The `Credential` and `Presentation` types both implement the [`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and [`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) traits from the [`serde` crate](https://crates.io/crates/serde). Hence one can use the [`serde_json` crate](https://crates.io/crates/serde_json) to obtain `Credential`s and `Presentation`s from JSON. 

### Example 
Deserializing a (verifiable) `Credential` from JSON. 
```rust
use identity_credential::credential::Credential;
use serde_json::json;

let credential_json: &'static str = r#"{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.gov/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": "https://example.edu",
  "issuanceDate": "2017-06-18T21:19:00Z",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science in Mechanical Engineering"
    }
  },
  "proof": {
    "type": "RsaSignature2018",
    "created": "2017-06-18T21:19:10Z",
    "proofPurpose": "assertionMethod",
    "verificationMethod": "https://example.com/jdoe/keys/1",
    "jws": "eyJhbGciOiJSUzI1NiIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..TCYt5XsITJX1CxPCT8yAV-TVkIEq_PbChOMqsLfRoPsnsgw5WEuts01mq-pQy7UJiN5mgRxD-WUcX16dUEMGlv50aqzpqh4Qktb3rk-BuQy72IFLOqV0G_zS245-kronKb78cPN25DGlcTwLtjPAYuNzVBAh4vGHSrQyHUdBBPM"
  }
}"#;

let credential: Credential = serde_json::from_str(credential_json).unwrap();

```

### Example 
Deserializing a (verifiable) `Presentation` from JSON. 

```rust
use identity_credential::presentation::Presentation;
use serde_json; 

let presentation_json: &'static str = r#"{
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "urn:uuid:3978344f-8596-4c3a-a978-8fcaba3903c5",
    "type": ["VerifiablePresentation", "CredentialManagerPresentation"],
    "verifiableCredential": [{
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.edu/credentials/3732",
      "type": ["VerifiableCredential", "UniversityDegreeCredential"],
      "issuer": "https://example.edu/issuers/14",
      "issuanceDate": "2010-01-01T19:23:24Z",
      "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "degree": {
          "type": "BachelorDegree",
          "name": "Bachelor of Science in Mechanical Engineering"
        }
      },
      "proof": {
        "type": "RsaSignature2018",
        "created": "2017-06-18T21:19:10Z",
        "proofPurpose": "assertionMethod",
        "verificationMethod": "https://example.com/jdoe/keys/1",
        "jws": "eyJhbGciOiJSUzI1NiIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..TCYt5XsITJX1CxPCT8yAV-TVkIEq_PbChOMqsLfRoPsnsgw5WEuts01mq-pQy7UJiN5mgRxD-WUcX16dUEMGlv50aqzpqh4Qktb3rk-BuQy72IFLOqV0G_zS245-kronKb78cPN25DGlcTwLtjPAYuNzVBAh4vGHSrQyHUdBBPM"
      }
    }],
    "proof": {
      "type": "RsaSignature2018",
      "created": "2017-06-18T21:19:10Z",
      "proofPurpose": "assertionMethod",
      "verificationMethod": "https://example.com/jdoe/keys/1",
      "jws": "eyJhbGciOiJSUzI1NiIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..TCYt5XsITJX1CxPCT8yAV-TVkIEq_PbChOMqsLfRoPsnsgw5WEuts01mq-pQy7UJiN5mgRxD-WUcX16dUEMGlv50aqzpqh4Qktb3rk-BuQy72IFLOqV0G_zS245-kronKb78cPN25DGlcTwLtjPAYuNzVBAh4vGHSrQyHUdBBPM"
    }
  }"#;

let presentation: Presentation = serde_json::from_str(presentation_json).unwrap();
  ```