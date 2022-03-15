use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActorRequest {
  pub endpoint: String,
  pub request: serde_json::Value,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IActorRequest")]
  pub type IActorRequest;
}

#[wasm_bindgen(typescript_custom_section)]
const I_ACTOR_REQUEST: &'static str = r#"
interface IActorRequest {
    readonly endpoint: string;
    readonly request: any;
}"#;
