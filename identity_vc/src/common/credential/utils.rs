use identity_core::common::{Object, Value};

use crate::{
    common::OneOrMany,
    error::{Error, Result},
};

pub fn take_object_id(object: &mut Object) -> Option<String> {
    match object.remove("id") {
        Some(Value::String(id)) => Some(id),
        Some(_) | None => None,
    }
}

pub fn try_take_object_id(name: &'static str, object: &mut Object) -> Result<String> {
    take_object_id(object).ok_or_else(|| Error::BadObjectConversion(name))
}

pub fn take_object_type(object: &mut Object) -> Option<String> {
    match object.remove("type") {
        Some(Value::String(value)) => Some(value.into()),
        Some(_) | None => None,
    }
}

pub fn try_take_object_type(name: &'static str, object: &mut Object) -> Result<String> {
    take_object_type(object).ok_or_else(|| Error::BadObjectConversion(name))
}

pub fn take_object_types(object: &mut Object) -> Option<OneOrMany<String>> {
    match object.remove("type") {
        Some(Value::String(value)) => Some(value.into()),
        Some(Value::Array(values)) => Some(collect_types(values)),
        Some(_) | None => None,
    }
}

pub fn try_take_object_types(name: &'static str, object: &mut Object) -> Result<OneOrMany<String>> {
    take_object_types(object).ok_or_else(|| Error::BadObjectConversion(name))
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
