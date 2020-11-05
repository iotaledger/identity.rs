/// Re-export `Value` from `serde_json` as the catch-all value type.
///
/// It's not ONLY a JSON value and implements Deserialize/Serialize.
pub use serde_json::Value;
