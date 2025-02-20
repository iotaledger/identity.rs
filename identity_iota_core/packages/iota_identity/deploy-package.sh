json_output=$(iota client publish --skip-dependency-verification --with-unpublished-dependencies --json --gas-budget 500000000 .)
echo $json_output
package_id=$(echo $json_output | jq --raw-output '.objectChanges[] | select(.type | contains("published")) | .packageId')
echo "IOTA_IDENTITY_PKG_ID=$package_id"