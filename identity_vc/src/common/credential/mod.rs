mod credential_schema;
mod credential_status;
mod credential_subject;
mod evidence;
mod refresh_service;
mod terms_of_use;

pub use self::{
  credential_schema::*, credential_status::*, credential_subject::*, evidence::*, refresh_service::*, terms_of_use::*,
};

use crate::common::{Object, OneOrMany, Value};

pub fn take_object_id(object: &mut Object) -> Option<String> {
  match object.remove("id") {
    Some(Value::String(id)) => Some(id),
    Some(_) => None,
    None => None,
  }
}

pub fn take_object_type(object: &mut Object) -> Option<OneOrMany<String>> {
  match object.remove("type") {
    Some(Value::String(value)) => Some(value.into()),
    Some(Value::Array(values)) => Some(collect_types(values)),
    Some(_) | None => None,
  }
}

fn collect_types(values: Vec<Value>) -> OneOrMany<String> {
  let mut types: Vec<String> = Vec::with_capacity(values.len());

  for value in values {
    if let Value::String(value) = value {
      types.push(value);
    }
  }

  types.into()
}
