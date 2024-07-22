use std::str::FromStr;

use iota_sdk::types::base_types::ObjectID;
use iota_sdk::types::Identifier;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;

use crate::Error;

pub fn identity_tag(package_id: ObjectID) -> Result<StructTag, Error> {
  Ok(StructTag {
    address: AccountAddress::from_str(&package_id.to_string())
      .map_err(|err| Error::ParsingFailed(format!("package id\"{package_id}\" to account address; {err}")))?,
    module: Identifier::from_str("identity")
      .map_err(|err| Error::ParsingFailed(format!("\"identity\" to identifier; {err}")))?,
    name: Identifier::from_str("Identity")
      .map_err(|err| Error::ParsingFailed(format!("\"Identity\" to identifier; {err}")))?,
    type_params: vec![],
  })
}
