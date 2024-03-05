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

pub trait StatusResolverT<S: StatusT> {
  type Error;

  async fn state<'c, S1>(&self, status: &'c S1) -> Result<S::State, Self::Error>
  where
    S: TryFrom<&'c S1>;
}

pub trait ValidableCredentialStatusExt<R, V, K>
where
  Self: ValidableCredential<R, V, K> + StatusCredentialT,
{
  async fn validate_with_status<'c, S, SR, F>(
    &'c self,
    resolver: &R,
    verifier: &V,
    status_resolver: &SR,
    state_predicate: F,
  ) -> Result<(), ()>
  where
    SR: StatusResolverT<S>,
    S: StatusT + TryFrom<&'c Self::Status>,
    F: FnOnce(&S::State) -> bool,
  {
    self.validate(resolver, verifier).await?;
    let Some(status) = self.status() else {
      return Ok(());
    };
    let credential_state = status_resolver.state(status).await.map_err(|_| ())?;

    if !state_predicate(&credential_state) {
      todo!("Non-valid state!")
    } else {
      Ok(())
    }
  }
}

impl<R, V, C, K> ValidableCredentialStatusExt<R, V, K> for C where C: ValidableCredential<R, V, K> + StatusCredentialT {}
