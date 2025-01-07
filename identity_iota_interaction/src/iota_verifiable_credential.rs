use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::{MoveType, TypedValue};
use crate::move_types::language_storage::TypeTag;
use crate::types::base_types::ObjectID;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotaVerifiableCredential {
    data: Vec<u8>,
}

impl IotaVerifiableCredential {
    pub fn new(data: Vec<u8>) -> IotaVerifiableCredential {
        IotaVerifiableCredential { data }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl MoveType for IotaVerifiableCredential {
    fn move_type(package: ObjectID) -> TypeTag {
        TypeTag::from_str(&format!("{package}::public_vc::PublicVc")).expect("valid utf8")
    }

    fn get_typed_value(&self, _package: ObjectID) -> TypedValue<Self> where Self: MoveType, Self: Sized {
        TypedValue::IotaVerifiableCredential(self)
    }
}