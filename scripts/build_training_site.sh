#!/usr/bin/env bash

set -euo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_root"

for command in mkdocs python3 quarto; do
    if ! command -v "$command" >/dev/null 2>&1; then
        echo "Required command not found: $command" >&2
        exit 127
    fi
done

echo "Building learning modules..."
mkdocs build --strict --site-dir "$repo_root/docs/learning-modules"

echo "Linking source-code references..."
python3 scripts/link_source_code_refs.py docs/learning-modules

echo "Building slide deck..."
slides_output_dir=$(mktemp -d "${TMPDIR:-/tmp}/rust-good-bad-ugly-slides.XXXXXX")
trap 'rm -rf "$slides_output_dir"' EXIT
quarto render slides-source/rust-good-bad-ugly.qmd \
    --to revealjs \
    --output-dir "$slides_output_dir"
mkdir -p docs/slides
cp -R "$slides_output_dir/." docs/slides/

echo "Generated training site assets under docs/."
