#!/bin/sh
cd ..

API_ENDPOINT="http://localhost" \
STRONGHOLD_PWD="secure_password" \
SNAPSHOT_PATH="/var/folders/j1/0sqjj6g12tzg3g7gsrqjmdxh0000gn/T/test_strongholds/fzb8vO38BeexyNMajzOsmtvvWd6ADL3s.stronghold" \
cargo +nightly run --release
