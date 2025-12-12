#!/bin/bash

TARGET_DIR="./target/wasm32-wasip1/release-wasi"

echo "--- Running int2dostime2date2str with VALID input ---"
cat examples/int_input.json | \
    wazero run "${TARGET_DIR}/int2dostime2date2str.wasm" | \
    jq .

echo -e "\n--- Running hex2be2dostime2date2str with VALID input ---"
cat examples/hex_be_input.json | \
    wazero run "${TARGET_DIR}/hex2be2dostime2date2str.wasm" | \
    jq .

echo -e "\n--- Running hex2le2dostime2date2str with VALID input ---"
cat examples/hex_le_input.json | \
    wazero run "${TARGET_DIR}/hex2le2dostime2date2str.wasm" | \
    jq .

echo -e "\n--- Testing INVALID inputs (using int2dostime2date2str for simplicity) ---"

run_invalid_test() {
    local input_file="$1"
    local description="$2"
    echo -e "\n--- Testing: ${description} ---"
    if ! output=$(wazero run "${TARGET_DIR}/int2dostime2date2str.wasm" < "${input_file}" 2>&1); then
        echo "Expected ERROR for ${description}:"
        echo "${output}"
    else
        echo "UNEXPECTED SUCCESS for ${description}. Output:"
        echo "${output}"
    fi
}

run_invalid_test "examples/invalid_month_0_input.json" "Month 0"
run_invalid_test "examples/invalid_month_13_input.json" "Month 13"
run_invalid_test "examples/invalid_day_0_input.json" "Day 0"
run_invalid_test "examples/invalid_feb_30_input.json" "February 30"
run_invalid_test "examples/invalid_apr_31_input.json" "April 31"
