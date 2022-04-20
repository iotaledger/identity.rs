// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Context;
use identity::core::Object;
use identity::core::OneOrMany;
use identity::core::Timestamp;
use identity::core::Url;
use identity::credential::CredentialBuilder;
use identity::credential::Evidence;
use identity::credential::Issuer;
use identity::credential::Policy;
use identity::credential::RefreshService;
use identity::credential::Schema;
use identity::credential::Status;
use identity::credential::Subject;
use wasm_bindgen::prelude::*;

use crate::error::WasmResult;

impl TryFrom<ICredential> for CredentialBuilder {
  type Error = JsValue;

  fn try_from(values: ICredential) -> std::result::Result<Self, Self::Error> {
    let ICredentialHelper {
      context,
      id,
      r#type,
      credential_subject,
      issuer,
      issuance_date,
      expiration_date,
      credential_status,
      credential_schema,
      refresh_service,
      terms_of_use,
      evidence,
      non_transferable,
      properties,
    } = values.into_serde::<ICredentialHelper>().wasm_result()?;

    let mut builder: CredentialBuilder = CredentialBuilder::new(properties);

    if let Some(context) = context {
      for value in context.into_vec() {
        builder = builder.context(value);
      }
    }
    if let Some(id) = id {
      builder = builder.id(Url::parse(&id).wasm_result()?);
    }
    if let Some(types) = r#type {
      for value in types.iter() {
        builder = builder.type_(value);
      }
    }
    if let Some(credential_subject) = credential_subject {
      for subject in credential_subject.into_vec() {
        builder = builder.subject(subject);
      }
    }
    if let Some(issuer) = issuer {
      builder = builder.issuer(issuer);
    }
    if let Some(issuance_date) = issuance_date {
      builder = builder.issuance_date(issuance_date);
    }
    if let Some(expiration_date) = expiration_date {
      builder = builder.expiration_date(expiration_date);
    }
    if let Some(credential_status) = credential_status {
      for status in credential_status.into_vec() {
        builder = builder.status(status);
      }
    }
    if let Some(credential_schema) = credential_schema {
      for schema in credential_schema.into_vec() {
        builder = builder.schema(schema);
      }
    }
    if let Some(refresh_service) = refresh_service {
      for service in refresh_service.into_vec() {
        builder = builder.refresh_service(service);
      }
    }
    if let Some(terms_of_use) = terms_of_use {
      for policy in terms_of_use.into_vec() {
        builder = builder.terms_of_use(policy);
      }
    }
    if let Some(evidence) = evidence {
      for value in evidence.into_vec() {
        builder = builder.evidence(value);
      }
    }
    if let Some(non_transferable) = non_transferable {
      builder = builder.non_transferable(non_transferable);
    }

    Ok(builder)
  }
}

/// Helper-struct for constructing a `Credential` by deserializing `ICredential` fields.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ICredentialHelper {
  context: Option<OneOrMany<Context>>,
  id: Option<String>,
  r#type: Option<OneOrMany<String>>,
  credential_subject: Option<OneOrMany<Subject>>,
  issuer: Option<Issuer>,
  issuance_date: Option<Timestamp>,
  expiration_date: Option<Timestamp>,
  credential_status: Option<OneOrMany<Status>>,
  credential_schema: Option<OneOrMany<Schema>>,
  refresh_service: Option<OneOrMany<RefreshService>>,
  terms_of_use: Option<OneOrMany<Policy>>,
  evidence: Option<OneOrMany<Evidence>>,
  non_transferable: Option<bool>,
  #[serde(flatten)]
  properties: Object,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ICredential")]
  pub type ICredential;
}

#[wasm_bindgen(typescript_custom_section)]
const I_CREDENTIAL: &'static str = r#"
/** Fields for constructing a new {@link Credential}. */
interface ICredential {
  /** The JSON-LD context(s) applicable to the `Credential`. */
  readonly context?: string | Record<string, any> | Array<string | Record<string, any>>;

  /** A unique URI referencing the subject of the `Credential`. */
  readonly id?: string;

  /** One or more URIs defining the type of the `Credential`. Contains base context by default. */
  readonly type?: string | Array<string>;

  /** One or more objects representing the `Credential` subject(s). */
  readonly credentialSubject: Subject | Array<Subject>;

  /** A reference to the issuer of the `Credential`. */
  readonly issuer: string | DID | Issuer;

  /** A timestamp of when the `Credential` becomes valid. Defaults to the current datetime. */
  readonly issuanceDate?: Timestamp;

  /** A timestamp of when the `Credential` should no longer be considered valid. */
  readonly expirationDate?: Timestamp;

  /** Information used to determine the current status of the `Credential`. */
  readonly credentialStatus?: Status | Array<Status>;

  /** Information used to assist in the enforcement of a specific `Credential` structure. */
  readonly credentialSchema?: Schema | Array<Schema>;

  /** Service(s) used to refresh an expired `Credential`. */
  readonly refreshService?: RefreshService | Array<RefreshService>;

  /** Terms-of-use specified by the `Credential` issuer. */
  readonly termsOfUse?: Policy | Array<Policy>;

  /** Human-readable evidence used to support the claims within the `Credential`. */
  readonly evidence?: Evidence | Array<Evidence>;

  /** Indicates that the `Credential` must only be contained within a {@link Presentation} with a proof issued from the `Credential` subject. */
  readonly nonTransferable?: boolean;

  /** Miscellaneous properties. */
  readonly [properties: string | symbol]: unknown;
}"#;

#[cfg(test)]
mod tests {
  use identity::core::FromJson;
  use identity::core::Object;
  use identity::credential::Credential;
  use wasm_bindgen::JsCast;
  use wasm_bindgen_test::*;

  use crate::credential::WasmCredential;

  use super::*;

  fn mock_credential_fields() -> JsValue {
    JsValue::from_serde(
      &Object::from_json(
        r#"{
  "context": "https://www.w3.org/2018/credentials/examples/v1",
  "id": "https://example.edu/credentials/3732",
  "type": "UniversityDegreeCredential",
  "credentialSubject": {
    "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts"
    }
  },
  "issuer": "https://example.edu/issuers/565049",
  "issuanceDate": "2010-01-01T00:00:00Z",
  "expirationDate": "2020-01-01T19:23:24Z",
  "credentialStatus": {
    "id": "https://example.edu/status/24",
    "type": "CredentialStatusList2017"
  },
  "credentialSchema": {
    "id": "https://example.org/examples/degree.json",
    "type": "JsonSchemaValidator2018"
  },
  "refreshService": {
    "id": "https://example.edu/refresh/3732",
    "type": "ManualRefreshService2018"
  },
  "termsOfUse": {
    "type": "IssuerPolicy",
    "id": "https://example.com/policies/credential/4",
    "profile": "https://example.com/profiles/credential",
    "prohibition": [{
      "assigner": "https://example.edu/issuers/14",
      "assignee": "AllVerifiers",
      "target": "https://example.edu/credentials/3732",
      "action": ["Archival"]
    }]
  },
  "evidence": {
    "id": "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192d4231",
    "type": ["DocumentVerification"],
    "verifier": "https://example.edu/issuers/14",
    "evidenceDocument": "DriversLicense",
    "subjectPresence": "Physical",
    "documentPresence": "Physical",
    "licenseNumber": "123AB4567"
  },
  "nonTransferable": true,
  "custom1": "asdf",
  "custom2": 1234
}"#,
      )
      .unwrap(),
    )
    .unwrap()
  }

  #[wasm_bindgen_test]
  fn test_credential_builder_try_from() {
    let json: JsValue = mock_credential_fields();
    let _builder: CredentialBuilder = CredentialBuilder::try_from(json.unchecked_into::<ICredential>()).unwrap();
  }

  #[wasm_bindgen_test]
  fn test_credential_constructor() {
    let json: JsValue = mock_credential_fields();
    let Credential {
      context,
      id,
      types,
      credential_subject,
      issuer,
      issuance_date,
      expiration_date,
      credential_status,
      credential_schema,
      refresh_service,
      terms_of_use,
      evidence,
      non_transferable,
      properties,
      proof,
    } = WasmCredential::new(json.unchecked_into()).unwrap().0;
    assert_eq!(
      context,
      OneOrMany::Many(vec![
        Context::Url(Url::parse("https://www.w3.org/2018/credentials/v1").unwrap()),
        Context::Url(Url::parse("https://www.w3.org/2018/credentials/examples/v1").unwrap()),
      ])
    );
    assert_eq!(id.unwrap(), "https://example.edu/credentials/3732");
    assert_eq!(
      types,
      OneOrMany::Many(vec![
        "VerifiableCredential".to_owned(),
        "UniversityDegreeCredential".to_owned()
      ])
    );
    assert_eq!(
      credential_subject,
      OneOrMany::One(
        Subject::from_json(
          r#"{
      "id":"did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(
      issuer,
      Issuer::Url(Url::parse("https://example.edu/issuers/565049").unwrap())
    );
    assert_eq!(issuance_date.to_string(), "2010-01-01T00:00:00Z");
    assert_eq!(expiration_date.unwrap().to_string(), "2020-01-01T19:23:24Z");
    assert_eq!(
      credential_status,
      OneOrMany::One(
        Status::from_json(
          r#"{
      "id": "https://example.edu/status/24",
      "type": "CredentialStatusList2017"
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(
      credential_schema,
      OneOrMany::One(
        Schema::from_json(
          r#"{
      "id": "https://example.org/examples/degree.json",
      "type": "JsonSchemaValidator2018"
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(
      refresh_service,
      OneOrMany::One(
        RefreshService::from_json(
          r#"{
      "id": "https://example.edu/refresh/3732",
      "type": "ManualRefreshService2018"
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(
      terms_of_use,
      OneOrMany::One(
        Policy::from_json(
          r#"{
      "type": "IssuerPolicy",
      "id": "https://example.com/policies/credential/4",
      "profile": "https://example.com/profiles/credential",
      "prohibition": [{
        "assigner": "https://example.edu/issuers/14",
        "assignee": "AllVerifiers",
        "target": "https://example.edu/credentials/3732",
        "action": ["Archival"]
      }]
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(
      evidence,
      OneOrMany::One(
        Evidence::from_json(
          r#"{
      "id": "https://example.edu/evidence/f2aeec97-fc0d-42bf-8ca7-0548192d4231",
      "type": ["DocumentVerification"],
      "verifier": "https://example.edu/issuers/14",
      "evidenceDocument": "DriversLicense",
      "subjectPresence": "Physical",
      "documentPresence": "Physical",
      "licenseNumber": "123AB4567"
    }"#
        )
        .unwrap()
      )
    );
    assert_eq!(non_transferable, Some(true));
    assert!(proof.is_none());

    let mut expected_properties: Object = Object::new();
    expected_properties.insert("custom1".into(), "asdf".into());
    expected_properties.insert("custom2".into(), 1234_i32.into());
    assert_eq!(properties, expected_properties);
  }
}
