use std::str::FromStr;

use iota_sdk::types::base_types::ObjectID;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;

use crate::utils::parse_identifier;
use crate::Error;

pub fn identity_tag(package_id: ObjectID) -> Result<StructTag, Error> {
  Ok(StructTag {
    address: AccountAddress::from_str(&package_id.to_string())
      .map_err(|err| Error::ParsingFailed(format!("package id\"{package_id}\" to account address; {err}")))?,
    module: parse_identifier("identity")?,
    name: parse_identifier("Identity")?,
    type_params: vec![],
  })
}
