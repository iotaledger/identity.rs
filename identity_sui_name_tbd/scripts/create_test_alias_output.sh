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
  --move-call iota::tx_context::sender \
  --assign sender \
  --make-move-vec "<u8>" "[68u8, 73u8, 68u8, 1u8, 0u8, 131u8, 1u8, 123u8, 34u8, 100u8, 111u8, 99u8, 34u8, 58u8, 123u8, 34u8, 105u8, 100u8, 34u8, 58u8, 34u8, 100u8, 105u8, 100u8, 58u8, 48u8, 58u8, 48u8, 34u8, 44u8, 34u8, 118u8, 101u8, 114u8, 105u8, 102u8, 105u8, 99u8, 97u8, 116u8, 105u8, 111u8, 110u8, 77u8, 101u8, 116u8, 104u8, 111u8, 100u8, 34u8, 58u8, 91u8, 123u8, 34u8, 105u8, 100u8, 34u8, 58u8, 34u8, 100u8, 105u8, 100u8, 58u8, 48u8, 58u8, 48u8, 35u8, 79u8, 115u8, 55u8, 95u8, 66u8, 100u8, 74u8, 120u8, 113u8, 86u8, 119u8, 101u8, 76u8, 107u8, 56u8, 73u8, 87u8, 45u8, 76u8, 71u8, 83u8, 111u8, 52u8, 95u8, 65u8, 115u8, 52u8, 106u8, 70u8, 70u8, 86u8, 113u8, 100u8, 108u8, 74u8, 73u8, 99u8, 48u8, 45u8, 100u8, 50u8, 49u8, 73u8, 34u8, 44u8, 34u8, 99u8, 111u8, 110u8, 116u8, 114u8, 111u8, 108u8, 108u8, 101u8, 114u8, 34u8, 58u8, 34u8, 100u8, 105u8, 100u8, 58u8, 48u8, 58u8, 48u8, 34u8, 44u8, 34u8, 116u8, 121u8, 112u8, 101u8, 34u8, 58u8, 34u8, 74u8, 115u8, 111u8, 110u8, 87u8, 101u8, 98u8, 75u8, 101u8, 121u8, 34u8, 44u8, 34u8, 112u8, 117u8, 98u8, 108u8, 105u8, 99u8, 75u8, 101u8, 121u8, 74u8, 119u8, 107u8, 34u8, 58u8, 123u8, 34u8, 107u8, 116u8, 121u8, 34u8, 58u8, 34u8, 79u8, 75u8, 80u8, 34u8, 44u8, 34u8, 97u8, 108u8, 103u8, 34u8, 58u8, 34u8, 69u8, 100u8, 68u8, 83u8, 65u8, 34u8, 44u8, 34u8, 107u8, 105u8, 100u8, 34u8, 58u8, 34u8, 79u8, 115u8, 55u8, 95u8, 66u8, 100u8, 74u8, 120u8, 113u8, 86u8, 119u8, 101u8, 76u8, 107u8, 56u8, 73u8, 87u8, 45u8, 76u8, 71u8, 83u8, 111u8, 52u8, 95u8, 65u8, 115u8, 52u8, 106u8, 70u8, 70u8, 86u8, 113u8, 100u8, 108u8, 74u8, 73u8, 99u8, 48u8, 45u8, 100u8, 50u8, 49u8, 73u8, 34u8, 44u8, 34u8, 99u8, 114u8, 118u8, 34u8, 58u8, 34u8, 69u8, 100u8, 50u8, 53u8, 53u8, 49u8, 57u8, 34u8, 44u8, 34u8, 120u8, 34u8, 58u8, 34u8, 75u8, 119u8, 99u8, 54u8, 89u8, 105u8, 121u8, 121u8, 65u8, 71u8, 79u8, 103u8, 95u8, 80u8, 116u8, 118u8, 50u8, 95u8, 49u8, 67u8, 80u8, 71u8, 52u8, 98u8, 86u8, 87u8, 54u8, 102u8, 89u8, 76u8, 80u8, 83u8, 108u8, 115u8, 57u8, 112u8, 122u8, 122u8, 99u8, 78u8, 67u8, 67u8, 77u8, 34u8, 125u8, 125u8, 93u8, 125u8, 44u8, 34u8, 109u8, 101u8, 116u8, 97u8, 34u8, 58u8, 123u8, 34u8, 99u8, 114u8, 101u8, 97u8, 116u8, 101u8, 100u8, 34u8, 58u8, 34u8, 50u8, 48u8, 50u8, 52u8, 45u8, 48u8, 53u8, 45u8, 50u8, 50u8, 84u8, 49u8, 50u8, 58u8, 49u8, 52u8, 58u8, 51u8, 50u8, 90u8, 34u8, 44u8, 34u8, 117u8, 112u8, 100u8, 97u8, 116u8, 101u8, 100u8, 34u8, 58u8, 34u8, 50u8, 48u8, 50u8, 52u8, 45u8, 48u8, 53u8, 45u8, 50u8, 50u8, 84u8, 49u8, 50u8, 58u8, 49u8, 52u8, 58u8, 51u8, 50u8, 90u8, 34u8, 125u8, 125u8]" \
  --assign state_metadata \
  --move-call "0x01::option::some<vector<u8>>" state_metadata \
  --assign state_metadata_option \
  --move-call $1::alias::create_for_testing \
    none \
    123u32 \
    state_metadata_option \
    none \
    none \
    none \
    none \
  --assign "alias" \
  --move-call $1::alias_output::create_empty_for_testing \
  --assign alias_output \
  --move-call $1::alias_output::attach_alias alias_output "alias" \
  --transfer-objects "[alias_output]" sender \
  --json 
