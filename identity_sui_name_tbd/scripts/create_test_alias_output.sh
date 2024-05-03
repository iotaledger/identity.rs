#!/bin/bash

# Copyright 2020-2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

if [ -z "$1" ]
  then
    echo "No arguments supplied, please pass package id as hex string"
    exit 1
fi


# create_for_testing args:
#        legacy_state_controller: Option<address>,
#        state_index: u32,
#        state_metadata: Option<vector<u8>>,
#        sender: Option<address>,
#        metadata: Option<vector<u8>>,
#        immutable_issuer: Option<address>,
#        immutable_metadata: Option<vector<u8>>,
#        ctx: &mut TxContext
sui \
  client \
    call \
      --package $1 \
      --module alias \
      --function create_for_testing \
      --args \
        [] \
        123 \
        [] \
        [] \
        [] \
        [] \
        [] \
      --gas-budget 10000000
