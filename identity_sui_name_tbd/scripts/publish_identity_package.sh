#!/bin/bash

# Copyright 2020-2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

script_dir=$(dirname $0)
package_dir=$script_dir/../packages/identity_iota

echo "publishing package from $package_dir"
cd $package_dir
sui client publish --gas-budget 100000000 .
