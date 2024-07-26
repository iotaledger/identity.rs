#!/bin/bash

# Copyright 2020-2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

if [ -z "$1" ]
  then
    echo "No arguments supplied, please pass identity package id as hex string"
    exit 1
fi

if [ -z "$2" ]
  then
    echo "pass the address of the alias output you want to migrate"
    exit 1
fi

if [ -z "$3" ]
  then
    echo "pass the address of the MigrationRegistry shared object"
    exit 1
fi

sui client ptb \
  --gas-budget 50000000 \
  --move-call iota::tx_context::sender \
  --move-call $1::migration::migrate_alias_output @$2 @$3 \
  --json
