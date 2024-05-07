#!/bin/bash

# Copyright 2020-2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

if [ -z "$1" ]
  then
    echo "No arguments supplied, please pass package id as hex string"
    exit 1
fi

sui client ptb \
  --gas-budget 50000000 \
  --move-call sui::tx_context::sender \
  --assign sender \
  --move-call $1::alias::create_for_testing \
    none \
    123u32 \
    'some("DIDwhatever")' \
    none \
    none \
    none \
    none \
  --assign "alias" \
  --move-call $1::alias_output::create_empty_for_testing \
  --assign alias_output \
  --move-call $1::alias_output::attach_alias alias_output "alias" \
  --transfer-objects "[alias_output]" sender 
