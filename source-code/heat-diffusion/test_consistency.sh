#!/usr/bin/env bash

set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

run_case() {
    local name=$1
    shift

    local naive_output="$tmp_dir/naive-$name.txt"
    local ndarray_output="$tmp_dir/ndarray-$name.txt"

    printf 'Checking %s...\n' "$name"

    cargo run --quiet \
        --manifest-path "$script_dir/naive/Cargo.toml" \
        -- "$@" >"$naive_output"

    cargo run --quiet \
        --manifest-path "$script_dir/ndarray-features/Cargo.toml" \
        -- "$@" >"$ndarray_output"

    if ! diff -u "$naive_output" "$ndarray_output"; then
        printf 'Consistency check failed for %s.\n' "$name" >&2
        return 1
    fi
}

run_case initial-state \
    --grid-size 11 \
    --spot-radius 2 \
    --steps 0 \
    --show

run_case one-step \
    --grid-size 11 \
    --spot-radius 2 \
    --alpha 0.1 \
    --dt 0.5 \
    --steps 1 \
    --tolerance 1.0e-12 \
    --show

run_case multi-step \
    --grid-size 21 \
    --spot-radius 5 \
    --alpha 0.1 \
    --dt 0.01 \
    --steps 25 \
    --tolerance 1.0e-12 \
    --show

run_case convergence \
    --grid-size 11 \
    --spot-radius 2 \
    --alpha 0.1 \
    --dt 0.5 \
    --steps 1000 \
    --tolerance 0.1 \
    --show

printf 'All cross-implementation consistency checks passed.\n'
