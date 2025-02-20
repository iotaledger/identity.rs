#!/usr/bin/env bash

set -o errexit   # abort on nonzero exitstatus
set -o nounset   # abort on unbound variable
set -o pipefail  # don't hide errors within pipes

json_output=$(iota client publish --skip-dependency-verification --with-unpublished-dependencies --json --gas-budget 500000000 .)
package_id=$(echo $json_output | jq --raw-output '.objectChanges[] | select(.type | contains("published")) | .packageId')
echo "IOTA_IDENTITY_PKG_ID=$package_id"