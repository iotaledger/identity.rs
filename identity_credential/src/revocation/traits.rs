use crate::credential::CredentialT;
use crate::credential::ValidableCredential;

pub trait StatusCredentialT: CredentialT {
  type Status;

  fn status(&self) -> Option<&Self::Status>;
}

pub trait StatusT {
  type State;

  fn type_(&self) -> &str;
}

pub trait StatusResolverT {
  type Error;
  type Status: StatusT;

  async fn state<'c, S>(&self, status: &'c S) -> Result<<Self::Status as StatusT>::State, Self::Error>
  where
    Self::Status: TryFrom<&'c S>;
}

pub trait ValidableCredentialStatusExt<R, V, K>
where
  Self: ValidableCredential<R, V, K> + StatusCredentialT,
{
  async fn validate_with_status<'c, SR, F>(
    &'c self,
    resolver: &R,
    verifier: &V,
    status_resolver: &SR,
    state_predicate: F,
  ) -> Result<(), ()>
  where
    SR: StatusResolverT,
    SR::Status: StatusT + TryFrom<&'c Self::Status>,
    F: FnOnce(&<SR::Status as StatusT>::State) -> bool,
  {
    self.validate(resolver, verifier).await?;
    let Some(status) = self.status() else {
      return Ok(());
    };
    let credential_state = status_resolver.state(status).await.map_err(|_| ())?;

    if !state_predicate(&credential_state) {
      Err(()) // TODO: return non-valid state
    } else {
      Ok(())
    }
  }
}

impl<R, V, C, K> ValidableCredentialStatusExt<R, V, K> for C where C: ValidableCredential<R, V, K> + StatusCredentialT {}
