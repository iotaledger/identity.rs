// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;
use iota_interaction::{ident_str, MoveType, TypedValue};
use iota_interaction::types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use iota_interaction::types::base_types::ObjectID;
use iota_interaction::types::transaction::{Argument, Command, ProgrammableMoveCall};
use crate::rebased::Error;

pub(crate) fn try_to_argument<T: MoveType + Serialize>(
    content: &T,
    ptb: &mut ProgrammableTransactionBuilder,
    package: ObjectID,
) -> Result<Argument, Error> {
    match content.get_typed_value(package) {
        TypedValue::IotaVerifiableCredential(value) => {
            let values = ptb
                .pure(value.data())
                .map_err(|e| Error::InvalidArgument(e.to_string()))?;
            Ok(ptb.command(Command::MoveCall(Box::new(ProgrammableMoveCall {
                package,
                module: ident_str!("public_vc").into(),
                function: ident_str!("new").into(),
                type_arguments: vec![],
                arguments: vec![values],
            }))))
        },
        TypedValue::Other(value) => {
            ptb.pure(value).map_err(|e| Error::InvalidArgument(e.to_string()))
        },
    }
}
