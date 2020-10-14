
use serde::{Deserialize, Serialize};

pub const TRUSTPING: &'static str = "https://didcomm.org/iota/v1/message_types/trustping";

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TrustPing {
    pub response_requested: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>
}

impl TrustPing {
    pub fn create() -> TrustPing {
        TrustPing::default()
    }

    pub fn set_comment(mut self, comment: Option<String>) -> TrustPing {
        self.comment = comment;
        self
    }

    pub fn request_response(mut self) -> TrustPing {
        self.response_requested = true;
        self
    }
}
