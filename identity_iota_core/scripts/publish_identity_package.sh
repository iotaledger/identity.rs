#!/bin/bash

# Copyright 2020-2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

script_dir=$(cd "$(dirname $0)" && pwd)
package_dir=$script_dir/../packages/iota_identity

echo "publishing package from $package_dir"
package_id=$(iota client publish --with-unpublished-dependencies --skip-dependency-verification --silence-warnings --json --gas-budget 500000000 $package_dir | jq --raw-output '.objectChanges[] | select(.type | contains("published")) | .packageId')
export IOTA_IDENTITY_PKG_ID=$package_id
