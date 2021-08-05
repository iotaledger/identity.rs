#[cfg(feature = "account")]
pub mod handler;
pub mod requests;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum StorageError {
    IotaError(String),
}

impl From<identity_iota::Error> for StorageError {
    fn from(err: identity_iota::Error) -> Self {
        Self::IotaError(format!("{}", err))
    }
}