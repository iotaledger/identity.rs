use crate::error::IdentityMessageError;
use identity_comm::did_comm::DIDComm;

use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe, UnwindSafe},
    result::Result,
};

use crate::message::{Message, MessageType, Response, ResponseType};

pub struct IdentityMessageHandler {
    // account_manager: AccountManager,
}

pub type IdentityMessageResult<T> = Result<T, IdentityMessageError>;

impl IdentityMessageHandler {
    pub fn new() -> IdentityMessageResult<Self> {
        let instance = Self {
        // account_manager: AccountManager::new()?,
      };
        Ok(instance)
    }

    pub async fn handle(&self, message: Message) {
        let response: Result<ResponseType, IdentityMessageError> = match message.message_type() {
            MessageType::TrustPing => convert_panics(|| self.handle_trust_ping()),
        };
        let response = match response {
            Ok(r) => r,
            // Err(e) => ResponseType::Error(e),
            Err(e) => ResponseType::Panic("ERROR".to_string()),
        };
        let _ = message
            .response_tx
            .send(Response::new(message.id(), message.message_type, response));

        // let _ = message.send();
    }

    /// Handle trust ping.
    fn handle_trust_ping(&self) -> Result<ResponseType, IdentityMessageError> {
        Ok(ResponseType::TrustPingResponse)
    }
}

impl Default for IdentityMessageHandler {
    fn default() -> Self {
        Self {
        // account_manager: AccountManager::new().unwrap(),
      }
    }
}

fn convert_panics<F: UnwindSafe + FnOnce() -> Result<ResponseType, IdentityMessageError>>(
    f: F,
) -> Result<ResponseType, IdentityMessageError> {
    match catch_unwind(|| f()) {
        Ok(result) => result,
        Err(panic) => panic_to_response_message(panic),
    }
}

fn panic_to_response_message(panic: Box<dyn Any>) -> Result<ResponseType, IdentityMessageError> {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = backtrace::Backtrace::new();
    Ok(ResponseType::Panic(format!("{}\n\n{:?}", msg, current_backtrace)))
}
