IOTA Identity - Credentials 
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

The generated `Credential` is not verifiable until it has been signed by the issuer's DID Document.

### Example - Presentation
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
A `Presentation` is not verifiable until signed by the holder's DID Document. All `Credentials` contained in the presentation must also be signed by their respective issuers. 

## JSON Serialization
The `Credential` and `Presentation` types both implement the [`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and [`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) traits from the [`serde` crate](https://crates.io/crates/serde). Hence one can use the [`serde_json` crate](https://crates.io/crates/serde_json) to obtain `Credential`s and `Presentation`s from JSON. 

### Example 
Deserializing a `Presentation` from JSON. 

```rust
use identity_credential::presentation::Presentation;
use serde_json; 

let presentation_json: &'static str = r#"{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "id": "http://example.org/credentials/3732",
  "type": "VerifiablePresentation",
  "verifiableCredential": [
    {
      "@context": "https://www.w3.org/2018/credentials/v1",
      "id": "https://example.edu/credentials/3732",
      "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
      ],
      "credentialSubject": {
        "id": "did:iota:4LCrrVYFQkYYn9VPejhebmMhNnCq24pYPao8yvVLwVje",
        "GPA": "4.0",
        "degree": {
          "name": "Bachelor of Science and Arts",
          "type": "BachelorDegree"
        },
        "name": "Alice"
      },
      "issuer": "did:iota:H3PBNPtLYkVaPpMQVz1R3LeT5zW1Hd6BXQmdtFptaGLR",
      "issuanceDate": "2019-01-01T00:00:00Z",
      "expirationDate": "2024-01-01T00:00:00Z",
      "nonTransferable": true,
      "proof": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "did:iota:H3PBNPtLYkVaPpMQVz1R3LeT5zW1Hd6BXQmdtFptaGLR#sign-0",
        "signatureValue": "5H2TSAG3cHnVEt7HZgg6aeYqmzKRQr9BTaP6mgHSE9uH9iLy7pK7TC2A5NHaiiFMGGaY3hJS5WUhfqCW3APxFhSP"
      }
    },
    {
      "@context": "https://www.w3.org/2018/credentials/v1",
      "id": "https://example.edu/credentials/3732",
      "type": [
        "VerifiableCredential",
        "UniversityDegreeCredential"
      ],
      "credentialSubject": {
        "id": "did:iota:b5DtNBzvJfz8jrX1FYYxgvHqvsoFofy1hxzPRMM5iH1",
        "GPA": "4.0",
        "degree": {
          "name": "Bachelor of Science and Arts",
          "type": "BachelorDegree"
        },
        "name": "Alice"
      },
      "issuer": "did:iota:7RD6LT5aSNuKMLJJYorGzhktpTG2TrxGSLmHnWW1Dbb",
      "issuanceDate": "2020-01-01T00:00:00Z",
      "expirationDate": "2023-01-01T00:00:00Z",
      "proof": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "did:iota:7RD6LT5aSNuKMLJJYorGzhktpTG2TrxGSLmHnWW1Dbb#sign-0",
        "signatureValue": "4QYkkDLDCZxfa6mymhGTGvG4NRgzdx5Txst7dM6jtfDpBV3Mif8hWH93RzR2MoVCtMgZf3ed7qoZsqepWkp4x9oU"
      }
    }
  ],
  "holder": "did:iota:4LCrrVYFQkYYn9VPejhebmMhNnCq24pYPao8yvVLwVje",
  "proof": {
    "type": "JcsEd25519Signature2020",
    "verificationMethod": "did:iota:4LCrrVYFQkYYn9VPejhebmMhNnCq24pYPao8yvVLwVje#sign-0",
    "signatureValue": "47YLi81cr8atfiyydTe4o989V8GBWZk6rVtvE5bAydhbd8HCK5c3wrNXRbBDAF8PDUBGGGqn8ZjA3jxGDFpQwGAW",
    "challenge": "475a7984-1bb5-4c4c-a56f-822bccd46440"
  }
}
"#;

let presentation: Presentation = serde_json::from_str(presentation_json).unwrap();
  ```