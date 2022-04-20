// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::Context;
use identity::core::Object;
use identity::core::OneOrMany;
use identity::core::Url;
use identity::credential::Credential;
use identity::credential::Policy;
use identity::credential::PresentationBuilder;
use identity::credential::RefreshService;
use wasm_bindgen::prelude::*;

use crate::error::WasmResult;

impl TryFrom<IPresentation> for PresentationBuilder {
  type Error = JsValue;

  fn try_from(values: IPresentation) -> std::result::Result<Self, Self::Error> {
    let IPresentationHelper {
      context,
      id,
      r#type,
      verifiable_credential,
      holder,
      refresh_service,
      terms_of_use,
      properties,
    } = values.into_serde::<IPresentationHelper>().wasm_result()?;

    let mut builder: PresentationBuilder = PresentationBuilder::new(properties);

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
    if let Some(credentials) = verifiable_credential {
      for credential in credentials.into_vec() {
        builder = builder.credential(credential);
      }
    }
    if let Some(holder) = holder {
      builder = builder.holder(Url::parse(holder).wasm_result()?);
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

    Ok(builder)
  }
}

/// Helper-struct for constructing a `Presentation` by deserializing `IPresentation` fields.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IPresentationHelper {
  context: Option<OneOrMany<Context>>,
  id: Option<String>,
  r#type: Option<OneOrMany<String>>,
  verifiable_credential: Option<OneOrMany<Credential>>,
  holder: Option<String>,
  refresh_service: Option<OneOrMany<RefreshService>>,
  terms_of_use: Option<OneOrMany<Policy>>,
  #[serde(flatten)]
  properties: Object,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IPresentation")]
  pub type IPresentation;
}

#[wasm_bindgen(typescript_custom_section)]
const I_PRESENTATION: &'static str = r#"
/** Fields for constructing a new {@link Presentation}. */
interface IPresentation {
  /** The JSON-LD context(s) applicable to the `Presentation`. */
  readonly context?: string | Record<string, any> | Array<string | Record<string, any>>;

  /** A unique URI of the `Presentation`. */
  readonly id?: string;

  /** One or more URIs defining the type of the `Presentation`. Contains the base context by default. */
  readonly type?: string | Array<string>;

  /** Credential(s) expressing the claims of the `Presentation`. */
  readonly verifiableCredential: Credential | Array<Credential>;

  /** The entity that generated the `Presentation`. */
  readonly holder?: string | DID;

  /** Service(s) used to refresh an expired {@link Credential} in the `Presentation`. */
  readonly refreshService?: RefreshService | Array<RefreshService>;

  /** Terms-of-use specified by the `Presentation` holder. */
  readonly termsOfUse?: Policy | Array<Policy>;

  /** Miscellaneous properties. */
  readonly [properties: string | symbol]: unknown;
}"#;
