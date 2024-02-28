use crate::credential::CredentialT;

pub trait StatusCredentialT: CredentialT {
  type Status;

  fn status(&self) -> Option<&Self::Status>;
  fn set_status(&mut self, status: Option<Self::Status>);
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
