use identity_core::{common::Url, ResolverT};

use crate::revocation::StatusResolverT;

use super::{StatusList2021Credential, StatusList2021Entry};

#[derive(Clone, Debug)]
pub struct StatusList2021Resolver<R>(R);

impl<R> StatusList2021Resolver<R> {
  pub const fn new(resolver: R) -> Self {
    Self(resolver)
  }
}

impl<R> StatusResolverT for StatusList2021Resolver<R>
where
  R: ResolverT<StatusList2021Credential>,
  R::Input: TryFrom<Url>,
{
  type Error = ();
  type Status = StatusList2021Entry;
  async fn state<'c, S>(
    &self,
    status: &'c S,
  ) -> Result<<Self::Status as crate::revocation::StatusT>::State, Self::Error>
  where
    Self::Status: TryFrom<&'c S>,
  {
    // Convert the provided status into a status we can work with.
    let status = Self::Status::try_from(status).map_err(|_| ())?;
    // Get the StatusList2021Credential's URL and convert it to something the resolver can work with.
    let credential_location = R::Input::try_from(status.status_list_credential().clone()).map_err(|_| ())?;
    // Fetch the credential.
    let credential = self.0.fetch(&credential_location).await.map_err(|_| ())?;

    // Return the entry specified in status
    credential.entry(status.index()).map_err(|_| ())
  }
}
