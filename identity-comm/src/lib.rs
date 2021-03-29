#[macro_use]
extern crate serde;

pub mod envelope;
pub mod error;
pub mod message;
pub mod utils;

pub mod builtin {
    pub struct TrustPing;

    impl TrustPing {
        pub const PING: &'static str = "https://didcomm.org/trust_ping/1.0/ping";
        pub const PING_RESPONSE: &'static str = "https://didcomm.org/trust_ping/1.0/ping_response";
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
    pub struct TrustPing_PING {
        pub response_requested: bool,
    }

    impl TrustPing_PING {
        pub const fn new(response_requested: bool) -> Self {
            Self { response_requested }
        }
    }
}
