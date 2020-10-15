use identity_crypto::SecretKey;

use crate::{did::DIDDiff, error::Result};

pub trait TangleDocument {
    fn sign_unchecked(&mut self, secret: &SecretKey) -> Result<()>;

    fn verify_unchecked(&self) -> Result<()>;

    fn sign_diff_unchecked(&self, diff: &mut DIDDiff, secret: &SecretKey) -> Result<()>;

    fn verify_diff_unchecked(&self, diff: &DIDDiff) -> Result<()>;
}
