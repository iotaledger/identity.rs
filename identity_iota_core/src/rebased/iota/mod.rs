// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod move_calls;
pub(crate) mod package;
pub(crate) mod types;

use std::collections::HashMap;
use std::collections::VecDeque;

use iota_sdk::graphql_client::Client;
use iota_sdk::transaction_builder::unresolved::Argument;
use iota_sdk::transaction_builder::unresolved::Command;
use iota_sdk::transaction_builder::unresolved::Input;
use iota_sdk::transaction_builder::unresolved::InputKind;
use iota_sdk::transaction_builder::TransactionBuilder;
use iota_sdk::types::ProgrammableTransaction;

pub(crate) fn ptb_merge_tx_with_inputs_replacement(
  ptb: &mut TransactionBuilder<Client>,
  other: ProgrammableTransaction,
  replacements: Vec<(Input, Argument)>,
) {
  let mut commands: VecDeque<Command> = other.commands.into_iter().map(Command::from).collect();

  // Move inputs over whilst applying replacements.
  let mut inputs_map = HashMap::with_capacity(other.inputs.len());
  for (idx, input) in other.inputs.into_iter().enumerate() {
    let argument = replacements
      .iter()
      .find_map(|(to_replace, replacement)| {
        resolved_and_unresolved_inputs_comparison(to_replace, &input).then_some(*replacement)
      })
      .unwrap_or_else(|| ptb.input(input));

    inputs_map.insert(idx as u16, argument);
  }

  // Move the first command over, obtaining the results offset to use.
  // Note: the very first command can only reference inputs as there
  //   aren't any results yet.
  let Some(mut fst_cmd) = commands.pop_front() else {
    // Transaction doesn't have any commands?
    return;
  };
  cmd_update_args(&mut fst_cmd, |arg| update_input_arg(arg, &inputs_map));
  let Argument::Result(offset) = ptb.command(fst_cmd) else {
    unreachable!("Ptb::command always returns a Result variant");
  };

  // Update `other` PT's commands by updating their inputs and arguments.
  commands.iter_mut().for_each(|cmd| {
    cmd_update_args(cmd, |arg| update_input_and_result(arg, &inputs_map, offset));
  });
  // Move the updated commands to PTB.
  for cmd in commands {
    ptb.command(cmd);
  }
}

#[cfg(test)]
#[inline]
pub(crate) fn ptb_merge_tx(ptb: &mut TransactionBuilder<Client>, other: ProgrammableTransaction) {
  ptb_merge_tx_with_inputs_replacement(ptb, other, vec![]);
}

fn update_input_arg(input_arg: &mut Argument, inputs_map: &HashMap<u16, Argument>) {
  let Argument::Input(idx) = input_arg else {
    return;
  };

  *input_arg = *inputs_map.get(&(*idx as u16)).expect("all inputs have been mapped");
}

fn update_input_and_result(arg: &mut Argument, inputs_map: &HashMap<u16, Argument>, result_offset: u16) {
  match arg {
    Argument::Input(_) => update_input_arg(arg, inputs_map),
    Argument::Result(idx) => *idx += result_offset,
    Argument::NestedResult(idx, _) => *idx += result_offset,
    _ => {}
  }
}

fn cmd_update_args<F>(cmd: &mut Command, update_fn: F)
where
  F: Fn(&mut Argument),
{
  let arguments = match cmd {
    Command::MoveCall(move_call) => move_call.arguments.iter_mut(),
    Command::TransferObjects(transfer) => {
      update_fn(&mut transfer.address);
      transfer.objects.iter_mut()
    }
    Command::MergeCoins(merge_coins) => {
      update_fn(&mut merge_coins.coin);
      merge_coins.coins_to_merge.iter_mut()
    }
    Command::SplitCoins(split_coins) => {
      update_fn(&mut split_coins.coin);
      split_coins.amounts.iter_mut()
    }
    Command::Upgrade(upgrade) => std::slice::from_mut(&mut upgrade.ticket).iter_mut(),
    Command::Publish(_) => std::slice::IterMut::default(),
    Command::MakeMoveVector(mmv) => mmv.elements.iter_mut(),
    _ => panic!("unsupported command variant"),
  };

  arguments.for_each(update_fn);
}

fn resolved_and_unresolved_inputs_comparison(a: &Input, b: &iota_sdk::types::Input) -> bool {
  // This compares pure values too!
  if let InputKind::Input(a) = &a.kind {
    a == b
  } else {
    a.object_id() == b.opt_object_id()
  }
}

#[cfg(test)]
mod tests {
  use iota_sdk::types::Address;
  use iota_sdk::types::ObjectId;
  use iota_sdk::types::SharedObjectReference;

  use super::*;

  /// Returns a PTB with a single call to `0x2::coin::zero`, together with its result.
  fn empty_iota_coin_ptb(sender: Address) -> (TransactionBuilder<Client>, Argument) {
    let mut ptb = TransactionBuilder::new(sender).with_client(Client::new_testnet());
    let empty_coin = ptb.move_call(ObjectId::FRAMEWORK, "coin", "zero").result();

    (ptb, empty_coin)
  }

  #[tokio::test]
  async fn merging_pt_into_empty_ptb_works() {
    let sender = Address::random();
    let mut ptb = TransactionBuilder::new(sender).with_client(Client::new_testnet());
    let pt = {
      let (mut ptb, coin) = empty_iota_coin_ptb(sender);
      ptb.transfer_objects(Address::random(), [coin]);
      ptb.finish_kind().await.unwrap().into_programmable()
    };

    ptb_merge_tx(&mut ptb, pt.clone());
    assert_eq!(ptb.finish_kind().await.unwrap().into_programmable(), pt);
  }

  #[tokio::test]
  async fn merging_pt_with_replacements_works() {
    let recipient = Address::random();
    let sender = Address::random();
    let object_to_replace = iota_sdk::types::Input::Shared(SharedObjectReference {
      object_id: ObjectId::random(),
      initial_shared_version: 0.into(),
      mutable: true,
    });
    // Base-PTB, where coin is the argument we'd like to use in the PT that we'll be merging.
    let (mut ptb, coin) = empty_iota_coin_ptb(sender);
    // In this PT we have two transfers: the first is a dummy that simply makes sure inputs and arguments
    // are handled as intented, the other is the transfer of an object that will be replaced with the
    // argument coming from the base PTB after the merge.
    let pt = {
      let mut ptb = TransactionBuilder::new(sender).with_client(Client::new_testnet());
      let pure_arg = ptb.pure_bytes(vec![1, 2, 3]);
      ptb.transfer_objects(recipient, [pure_arg]);

      let obj = ptb.input(object_to_replace.clone());
      ptb.transfer_objects(recipient, [obj]);
      ptb.finish_kind().await.unwrap().into_programmable()
    };

    ptb_merge_tx_with_inputs_replacement(
      &mut ptb,
      pt,
      vec![(
        Input {
          kind: InputKind::Input(object_to_replace),
          is_gas: false,
        },
        coin,
      )],
    );
    let pt = ptb.finish().await.unwrap();

    // What the PT should look like if created in a single PTB.
    let expected_pt = {
      let (mut ptb, coin) = empty_iota_coin_ptb(sender);
      let pure_arg = ptb.pure_bytes(vec![1, 2, 3]);
      ptb.transfer_objects(recipient, [pure_arg]);
      ptb.transfer_objects(recipient, [coin]);

      ptb.finish().await.unwrap()
    };

    assert_eq!(pt, expected_pt);
    assert_eq!(pt.as_v1().kind.as_programmable().inputs.len(), 2);
  }
}
