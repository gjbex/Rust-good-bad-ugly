#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

WARMUP="${WARMUP:-3}"
RUNS="${RUNS:-10}"
WIDTH="${WIDTH:-800}"
HEIGHT="${HEIGHT:-800}"
MAX_ITERATIONS="${MAX_ITERATIONS:-1000}"
C_REAL="${C_REAL:--0.5125}"
C_IMAG="${C_IMAG:-0.5213}"
EXPORT_JSON="${EXPORT_JSON:-benchmark-rust-cpp-results.json}"

RUST_MANIFEST="$ROOT_DIR/julia-set-baseline/Cargo.toml"
RUST_BINARY="$ROOT_DIR/julia-set-baseline/target/release/julia-set-baseline"
CPP_SOURCE_DIR="$ROOT_DIR/julia-set-cpp"
CPP_BUILD_DIR="$CPP_SOURCE_DIR/build"
CPP_BINARY="$CPP_BUILD_DIR/julia-set-cpp"

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo is required but was not found in PATH" >&2
    exit 1
fi

if ! command -v cmake >/dev/null 2>&1; then
    echo "error: cmake is required but was not found in PATH" >&2
    exit 1
fi

if ! command -v hyperfine >/dev/null 2>&1; then
    echo "error: hyperfine is required but was not found in PATH" >&2
    exit 1
fi

echo "Building julia-set-baseline"
cargo build --release --manifest-path "$RUST_MANIFEST"

if [[ ! -x "$RUST_BINARY" ]]; then
    echo "error: expected executable not found: $RUST_BINARY" >&2
    exit 1
fi

echo "Building julia-set-cpp"
cmake -S "$CPP_SOURCE_DIR" -B "$CPP_BUILD_DIR" -DCMAKE_BUILD_TYPE=Release
cmake --build "$CPP_BUILD_DIR"

if [[ ! -x "$CPP_BINARY" ]]; then
    echo "error: expected executable not found: $CPP_BINARY" >&2
    exit 1
fi

echo "Smoke testing julia-set-baseline"
"$RUST_BINARY" --width 8 --height 6 --max-iterations 20 --c-real="$C_REAL" --c-imag="$C_IMAG" >/dev/null

echo "Smoke testing julia-set-cpp"
"$CPP_BINARY" --width 8 --height 6 --max-iterations 20 --c-real="$C_REAL" --c-imag="$C_IMAG" >/dev/null

printf -v rust_command '%q --width %q --height %q --max-iterations %q --c-real=%q --c-imag=%q > /dev/null' \
    "$RUST_BINARY" "$WIDTH" "$HEIGHT" "$MAX_ITERATIONS" "$C_REAL" "$C_IMAG"
printf -v cpp_command '%q --width %q --height %q --max-iterations %q --c-real=%q --c-imag=%q > /dev/null' \
    "$CPP_BINARY" "$WIDTH" "$HEIGHT" "$MAX_ITERATIONS" "$C_REAL" "$C_IMAG"

echo "Running Rust vs. C++ benchmark"
echo "  warmup:         $WARMUP"
echo "  runs:           $RUNS"
echo "  width:          $WIDTH"
echo "  height:         $HEIGHT"
echo "  max iterations: $MAX_ITERATIONS"
echo "  c:              $C_REAL + ${C_IMAG}i"
echo "  export JSON:    $EXPORT_JSON"

hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --export-json "$EXPORT_JSON" \
    "$rust_command" \
    "$cpp_command"
