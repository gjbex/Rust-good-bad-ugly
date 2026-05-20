#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

WARMUP="${WARMUP:-3}"
RUNS="${RUNS:-10}"
NUM_PARTICLES="${NUM_PARTICLES:-200}"
STEPS="${STEPS:-500}"
DELTA_TIME="${DELTA_TIME:-0.001}"
SOFTENING="${SOFTENING:-0.01}"
SEED="${SEED:-1234}"
EXPORT_JSON="${EXPORT_JSON:-benchmark-rust-cpp-results.json}"

RUST_MANIFEST="$ROOT_DIR/rust/Cargo.toml"
RUST_BINARY="$ROOT_DIR/rust/target/release/n-body-simulation"
CPP_SOURCE_DIR="$ROOT_DIR/cpp"
CPP_BUILD_DIR="$CPP_SOURCE_DIR/build"
CPP_BINARY="$CPP_BUILD_DIR/n-body-simulation-cpp"

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

echo "Building n-body-simulation Rust"
cargo build --release --manifest-path "$RUST_MANIFEST"

if [[ ! -x "$RUST_BINARY" ]]; then
    echo "error: expected executable not found: $RUST_BINARY" >&2
    exit 1
fi

echo "Building n-body-simulation C++"
cmake -S "$CPP_SOURCE_DIR" -B "$CPP_BUILD_DIR" -DCMAKE_BUILD_TYPE=Release
cmake --build "$CPP_BUILD_DIR"

if [[ ! -x "$CPP_BINARY" ]]; then
    echo "error: expected executable not found: $CPP_BINARY" >&2
    exit 1
fi

echo "Smoke testing n-body-simulation Rust"
"$RUST_BINARY" --num-particles 3 --steps 2 --seed "$SEED" --delta-time "$DELTA_TIME" --softening "$SOFTENING" >/dev/null

echo "Smoke testing n-body-simulation C++"
"$CPP_BINARY" --num-particles 3 --steps 2 --seed "$SEED" --delta-time "$DELTA_TIME" --softening "$SOFTENING" >/dev/null

printf -v rust_command '%q --num-particles %q --steps %q --seed %q --delta-time %q --softening %q > /dev/null' \
    "$RUST_BINARY" "$NUM_PARTICLES" "$STEPS" "$SEED" "$DELTA_TIME" "$SOFTENING"
printf -v cpp_command '%q --num-particles %q --steps %q --seed %q --delta-time %q --softening %q > /dev/null' \
    "$CPP_BINARY" "$NUM_PARTICLES" "$STEPS" "$SEED" "$DELTA_TIME" "$SOFTENING"

echo "Running Rust vs. C++ n-body benchmark"
echo "  warmup:       $WARMUP"
echo "  runs:         $RUNS"
echo "  particles:    $NUM_PARTICLES"
echo "  steps:        $STEPS"
echo "  delta time:   $DELTA_TIME"
echo "  softening:    $SOFTENING"
echo "  seed:         $SEED"
echo "  export JSON:  $EXPORT_JSON"

hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --export-json "$EXPORT_JSON" \
    "$rust_command" \
    "$cpp_command"
