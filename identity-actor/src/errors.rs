pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota actor crate.
pub enum Error {
    #[error("Lock In Use")]
    LockInUse
}