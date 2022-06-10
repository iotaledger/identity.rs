# Iota Identity: Credentials 

This crate contains types representing *credentials* and *presentations* defined in the [W3C VC-Data model]((https://www.w3.org/TR/vc-data-model/). An overview of these concepts and how to create *verifiable* credentials and presentations from the building blocks in this crate is explained in [the project's wiki](https://wiki.iota.org/identity.rs/concepts/verifiable_credentials/overview). 

## Using the builders 
This crate enables creation of [`Credential`s](crate::credential::Credential) and [`Presentation`s](crate::presentation::Presentation) using the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html). 
### Example
Constructing a [`Credential`](crate::credential::Credential) using the [`CredentialBuilder`](crate::credential::CredentialBuilder). 

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
The `Credential` and `Presentation` types both implement the [`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and [`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) traits from the [`serde` crate](https://crates.io/crates/serde) hence one can use (for instance) the [`serde_json` crate](https://crates.io/crates/serde_json) to obtain `Credential`s and `Presentation`s from JSON. 

### Example 
Deserializing a `Credential` from JSON. 


### Example 
Deserializing a `Presentation` from JSON. 
