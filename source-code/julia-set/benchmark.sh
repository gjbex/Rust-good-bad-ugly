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
EXPORT_JSON="${EXPORT_JSON:-benchmark-results.json}"
EXCLUDE_RAYON=false

usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Options:
  --exclude-rayon  Skip the julia-set-rayon implementation
  -h, --help       Print this help message

Benchmark settings are configured with environment variables:
  WARMUP, RUNS, WIDTH, HEIGHT, MAX_ITERATIONS, C_REAL, C_IMAG, EXPORT_JSON
EOF
}

while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --exclude-rayon)
            EXCLUDE_RAYON=true
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown option: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
    shift
done

write_config() {
    local file="$1"
    local max_iterations="$2"
    local width="$3"
    local height="$4"
    local c_real="$5"
    local c_imag="$6"

    cat >"$file" <<EOF
max_iterations = $max_iterations
width = $width
height = $height
c_real = $c_real
c_imag = $c_imag
EOF
}

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo is required but was not found in PATH" >&2
    exit 1
fi

if ! command -v hyperfine >/dev/null 2>&1; then
    echo "error: hyperfine is required but was not found in PATH" >&2
    exit 1
fi

mapfile -t package_dirs < <(find . -mindepth 2 -maxdepth 2 -name Cargo.toml -printf '%h\n' | sort)

if [[ "${#package_dirs[@]}" -eq 0 ]]; then
    echo "error: no Cargo packages found below $ROOT_DIR" >&2
    exit 1
fi

commands=()
benchmark_config="$(mktemp "$ROOT_DIR/.benchmark-config.XXXXXX.toml")"
smoke_config="$(mktemp "$ROOT_DIR/.smoke-config.XXXXXX.toml")"
trap 'rm -f "$benchmark_config" "$smoke_config"' EXIT

write_config "$benchmark_config" "$MAX_ITERATIONS" "$WIDTH" "$HEIGHT" "$C_REAL" "$C_IMAG"
write_config "$smoke_config" 20 8 6 "$C_REAL" "$C_IMAG"

for dir in "${package_dirs[@]}"; do
    manifest="$dir/Cargo.toml"
    package_name="$(sed -n 's/^name = "\(.*\)"/\1/p' "$manifest" | head -n 1)"

    if [[ -z "$package_name" ]]; then
        echo "error: could not determine package name from $manifest" >&2
        exit 1
    fi

    if [[ "$EXCLUDE_RAYON" == true && "$package_name" == "julia-set-rayon" ]]; then
        echo "Skipping $package_name"
        continue
    fi

    echo "Building $package_name"
    cargo build --release --manifest-path "$manifest"

    binary="$dir/target/release/$package_name"
    if [[ ! -x "$binary" ]]; then
        echo "error: expected executable not found: $binary" >&2
        exit 1
    fi

    echo "Smoke testing $package_name"
    if [[ "$package_name" == "julia-set-toml-config" ]]; then
        "$binary" "$smoke_config" >/dev/null
        printf -v command '%q %q > /dev/null' "$binary" "$benchmark_config"
    else
        "$binary" --width 8 --height 6 --max-iterations 20 --c-real="$C_REAL" --c-imag="$C_IMAG" >/dev/null
        printf -v command '%q --width %q --height %q --max-iterations %q --c-real=%q --c-imag=%q > /dev/null' \
            "$binary" "$WIDTH" "$HEIGHT" "$MAX_ITERATIONS" "$C_REAL" "$C_IMAG"
    fi
    commands+=("$command")
done

echo "Running benchmarks"
echo "  warmup:         $WARMUP"
echo "  runs:           $RUNS"
echo "  width:          $WIDTH"
echo "  height:         $HEIGHT"
echo "  max iterations: $MAX_ITERATIONS"
echo "  c:              $C_REAL + ${C_IMAG}i"
echo "  export JSON:    $EXPORT_JSON"
echo "  exclude Rayon:  $EXCLUDE_RAYON"

hyperfine \
    --warmup "$WARMUP" \
    --runs "$RUNS" \
    --export-json "$EXPORT_JSON" \
    "${commands[@]}"
