use identity_common::impl_error_ctor;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid key: `{0}`")]
    KeyError(anyhow::Error),
    #[error("Failed to create signature: {0}")]
    SignError(anyhow::Error),
    #[error("Failed to verify signature: {0}")]
    VerifyError(anyhow::Error),
    #[error("Failed to create proof: {0}")]
    CreateProof(anyhow::Error),
    #[error("Failed to verify proof: {0}")]
    VerifyProof(anyhow::Error),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

impl Error {
    impl_error_ctor!(key_error, KeyError, Into<anyhow::Error>);
    impl_error_ctor!(sign_error, SignError, Into<anyhow::Error>);
    impl_error_ctor!(verify_error, VerifyError, Into<anyhow::Error>);
    impl_error_ctor!(create_proof, CreateProof, Into<anyhow::Error>);
    impl_error_ctor!(verify_proof, VerifyProof, Into<anyhow::Error>);
    impl_error_ctor!(custom, Custom, Into<anyhow::Error>);
}

pub type Result<T, E = Error> = anyhow::Result<T, E>;
