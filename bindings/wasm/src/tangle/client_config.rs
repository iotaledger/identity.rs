// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use identity::iota::ClientBuilder;
use identity::iota::DIDMessageEncoding;
use identity::iota_core::Network;
use proc_typescript::typescript;
use wasm_bindgen::prelude::*;

use crate::error::WasmResult;
use crate::tangle::WasmDIDMessageEncoding;

/// Try construct a `ClientBuilder` directly from an `IClientConfig` interface.
impl TryFrom<IClientConfig> for ClientBuilder {
  type Error = JsValue;

  fn try_from(config: IClientConfig) -> std::result::Result<Self, Self::Error> {
    let ClientConfig {
      network,
      encoding,
      nodes,
      primary_node,
      primary_pow_node,
      permanodes,
      node_auth,
      node_sync_interval,
      node_sync_disabled,
      quorum,
      quorum_size,
      quorum_threshold,
      local_pow,
      fallback_to_local_pow,
      tips_interval,
      request_timeout,
    } = config.into_serde::<ClientConfig>().wasm_result()?;

    let mut builder: ClientBuilder = ClientBuilder::new();
    if let Some(network) = network {
      builder = builder.network(network);
    }
    if let Some(encoding) = encoding {
      builder = builder.encoding(DIDMessageEncoding::from(encoding));
    }
    if let Some(nodes) = nodes {
      builder = builder
        .nodes(&nodes.iter().map(AsRef::as_ref).collect::<Vec<_>>())
        .wasm_result()?;
    }
    if let Some(NodeAuth {
      url,
      jwt,
      username,
      password,
    }) = primary_node
    {
      builder = builder
        .primary_node(&url, jwt, basic_auth(&username, &password))
        .wasm_result()?;
    }
    if let Some(NodeAuth {
      url,
      jwt,
      username,
      password,
    }) = primary_pow_node
    {
      builder = builder
        .primary_pow_node(&url, jwt, basic_auth(&username, &password))
        .wasm_result()?;
    }
    for NodeAuth {
      url,
      jwt,
      username,
      password,
    } in permanodes.unwrap_or_default()
    {
      builder = builder
        .permanode(&url, jwt, basic_auth(&username, &password))
        .wasm_result()?;
    }
    for NodeAuth {
      url,
      jwt,
      username,
      password,
    } in node_auth.unwrap_or_default()
    {
      builder = builder
        .node_auth(&url, jwt, basic_auth(&username, &password))
        .wasm_result()?;
    }
    if let Some(node_sync_interval) = node_sync_interval {
      builder = builder.node_sync_interval(Duration::from_secs(u64::from(node_sync_interval)));
    }
    if let Some(node_sync_disabled) = node_sync_disabled {
      if node_sync_disabled {
        builder = builder.node_sync_disabled();
      }
    }
    if let Some(quorum) = quorum {
      builder = builder.quorum(quorum);
    }
    if let Some(quorum_size) = quorum_size {
      builder = builder.quorum_size(quorum_size);
    }
    if let Some(quorum_threshold) = quorum_threshold {
      builder = builder.quorum_threshold(quorum_threshold);
    }
    if let Some(local_pow) = local_pow {
      builder = builder.local_pow(local_pow);
    }
    if let Some(fallback_to_local_pow) = fallback_to_local_pow {
      builder = builder.fallback_to_local_pow(fallback_to_local_pow);
    }
    if let Some(tips_interval) = tips_interval {
      builder = builder.tips_interval(u64::from(tips_interval));
    }
    if let Some(request_timeout) = request_timeout {
      builder = builder.request_timeout(Duration::from_secs(u64::from(request_timeout)));
    }

    Ok(builder)
  }
}

/// Helper function to combine a username and password into a basic authentication tuple.
fn basic_auth<'a>(username: &'a Option<String>, password: &'a Option<String>) -> Option<(&'a str, &'a str)> {
  username.as_deref().zip(password.as_deref())
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IClientConfig")]
  pub type IClientConfig;
}

/// Helper-struct for deserializing [`INodeAuth`].
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct NodeAuth {
  url: String,
  jwt: Option<String>,
  username: Option<String>,
  password: Option<String>,
}

#[wasm_bindgen(typescript_custom_section)]
const I_NODE_AUTH: &'static str = r#"
/** IOTA node details with optional authentication. */
interface INodeAuth {
    readonly url: string;
    readonly jwt?: string;
    readonly username?: string;
    readonly password?: string;
}"#;

/// {@link Client} configuration options.
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[typescript(name = "IClientConfig", readonly, optional)]
struct ClientConfig {
  /// Sets the IOTA Tangle network.
  #[typescript(type = "Network")]
  network: Option<Network>,
  /// Sets the DID message encoding used when publishing to the Tangle.
  #[typescript(type = "DIDMessageEncoding")]
  encoding: Option<WasmDIDMessageEncoding>,
  /// Adds a list of IOTA nodes to use by their URLs.
  #[typescript(type = "Array<string>")]
  nodes: Option<Vec<String>>,
  /// Sets an IOTA node by its URL to be used as primary node.
  #[typescript(name = "primaryNode", type = "INodeAuth")]
  primary_node: Option<NodeAuth>,
  /// Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW).
  #[typescript(name = "primaryPowNode", type = "INodeAuth")]
  primary_pow_node: Option<NodeAuth>,
  /// Adds a list of IOTA permanodes by their URLs.
  #[typescript(type = "Array<INodeAuth>")]
  permanodes: Option<Vec<NodeAuth>>,
  /// Adds a list of IOTA nodes to be used by their URLs.
  #[typescript(name = "nodeAuth", type = "Array<INodeAuth>")]
  node_auth: Option<Vec<NodeAuth>>,
  /// Sets the node sync interval in seconds.
  #[typescript(name = "nodeSyncInterval", type = "number")]
  node_sync_interval: Option<u32>,
  /// Disables the node sync process.
  #[typescript(name = "nodeSyncDisabled", type = "boolean")]
  node_sync_disabled: Option<bool>,
  /// Enables/disables quorum.
  #[typescript(type = "boolean")]
  quorum: Option<bool>,
  /// Sets the number of nodes used for quorum.
  #[typescript(name = "quorumSize", type = "number")]
  quorum_size: Option<usize>,
  /// Sets the quorum threshold.
  #[typescript(name = "quorumThreshold", type = "number")]
  quorum_threshold: Option<usize>,
  /// Sets whether proof-of-work (PoW) is performed locally or remotely.
  /// Default: false.
  #[typescript(name = "localPow", type = "boolean")]
  local_pow: Option<bool>,
  /// Sets whether the PoW should be done locally in case a node doesn't support remote PoW.
  /// Default: true.
  #[typescript(name = "fallbackToLocalPow", type = "boolean")]
  fallback_to_local_pow: Option<bool>,
  /// Sets the number of seconds that new tips will be requested during PoW.
  #[typescript(name = "tipsInterval", type = "number")]
  tips_interval: Option<u32>,
  /// Sets the default request timeout.
  #[typescript(name = "requestTimeout", type = "number")]
  request_timeout: Option<u32>,
}

#[cfg(test)]
mod tests {
  use identity::core::FromJson;
  use identity::core::Object;
  use wasm_bindgen::JsCast;
  use wasm_bindgen_test::*;

  use super::*;

  fn mock_client_config_json() -> JsValue {
    JsValue::from_serde(
      &Object::from_json(
        r#"{
      "network": "dev",
      "encoding": 1,
      "nodes": ["https://example.com:1", "https://example.com:2"],
      "primaryNode": {
        "url": "https://example.com:3",
        "username": "user",
        "password": "pass"
      },
      "primaryPowNode": {
        "url": "https://example.com:4"
      },
      "permanodes": [{ "url": "https://example.com:5" }, { "url": "https://example.com:6" }],
      "nodeAuth": [{ "url": "https://example.com:7" }, { "url": "https://example.com:8" }],
      "nodeSyncInterval": 42,
      "nodeSyncDisabled": true,
      "quorum": true,
      "quorumSize": 3,
      "quorumThreshold": 2,
      "localPow": false,
      "fallbackToLocalPow": false,
      "tipsInterval": 7,
      "requestTimeout": 60
    }"#,
      )
      .unwrap(),
    )
    .unwrap()
  }

  #[wasm_bindgen_test]
  fn test_client_config_try_from() {
    let json: JsValue = mock_client_config_json();
    let _client_builder: ClientBuilder = ClientBuilder::try_from(json.unchecked_into::<IClientConfig>()).unwrap();
  }

  #[wasm_bindgen_test]
  fn test_client_config_serde() {
    let json: JsValue = mock_client_config_json();
    let ClientConfig {
      network,
      encoding,
      nodes,
      primary_node,
      primary_pow_node,
      permanodes,
      node_auth,
      node_sync_interval,
      node_sync_disabled,
      quorum,
      quorum_size,
      quorum_threshold,
      local_pow,
      fallback_to_local_pow,
      tips_interval,
      request_timeout,
    } = json.into_serde::<ClientConfig>().unwrap();
    assert_eq!(network, Some(Network::Devnet));
    assert_eq!(
      encoding.map(DIDMessageEncoding::from),
      Some(DIDMessageEncoding::JsonBrotli)
    );
    assert_eq!(
      nodes,
      Some(vec![
        "https://example.com:1".to_owned(),
        "https://example.com:2".to_owned(),
      ])
    );
    assert_eq!(
      primary_node,
      Some(NodeAuth {
        url: "https://example.com:3".to_owned(),
        jwt: None,
        username: Some("user".to_owned()),
        password: Some("pass".to_owned()),
      })
    );
    assert_eq!(
      primary_pow_node,
      Some(NodeAuth {
        url: "https://example.com:4".to_owned(),
        jwt: None,
        username: None,
        password: None,
      })
    );
    assert_eq!(
      permanodes,
      Some(vec![
        NodeAuth {
          url: "https://example.com:5".to_owned(),
          jwt: None,
          username: None,
          password: None,
        },
        NodeAuth {
          url: "https://example.com:6".to_owned(),
          jwt: None,
          username: None,
          password: None,
        },
      ])
    );
    assert_eq!(
      node_auth,
      Some(vec![
        NodeAuth {
          url: "https://example.com:7".to_owned(),
          jwt: None,
          username: None,
          password: None,
        },
        NodeAuth {
          url: "https://example.com:8".to_owned(),
          jwt: None,
          username: None,
          password: None,
        },
      ])
    );
    assert_eq!(node_sync_interval, Some(42));
    assert_eq!(node_sync_disabled, Some(true));
    assert_eq!(quorum, Some(true));
    assert_eq!(quorum_size, Some(3));
    assert_eq!(quorum_threshold, Some(2));
    assert_eq!(local_pow, Some(false));
    assert_eq!(fallback_to_local_pow, Some(false));
    assert_eq!(tips_interval, Some(7));
    assert_eq!(request_timeout, Some(60));
  }
}
