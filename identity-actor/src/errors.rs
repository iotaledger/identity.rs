pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the identity actor crate.
pub enum Error {
    #[error("Lock In Use")]
    LockInUse,
    #[error("{0}")]
    OutboundFailure(#[from] communication_refactored::OutboundFailure),
    #[error("Unexpected Request")]
    UnexpectedRequest,
}
