#!/usr/bin/env python3
"""Link generated source-code references to the GitHub repository.

MkDocs renders inline code spans such as `source-code/math` as
<code>source-code/math</code>.  The learning-module sources should remain plain
Markdown, so this script post-processes the generated HTML and links those
inline references to the corresponding path on GitHub.
"""

from __future__ import annotations

import argparse
import html
import os
import re
from pathlib import Path
from urllib.parse import quote


PRE_BLOCK_RE = re.compile(r"(<pre\b.*?</pre>)", re.DOTALL | re.IGNORECASE)
SOURCE_CODE_RE = re.compile(r"<code>(source-code(?:/[^<\s]*)?)</code>")


def source_url(path_text: str, repo_url: str, branch: str, repo_root: Path) -> str | None:
    """Return the GitHub URL for a source-code path, or None if it is unknown."""
    path = Path(path_text)
    local_path = repo_root / path
    if local_path.is_dir():
        view = "tree"
    elif local_path.is_file():
        view = "blob"
    else:
        return None
    quoted_path = quote(path.as_posix(), safe="/-._~")
    return f"{repo_url.rstrip('/')}/{view}/{quote(branch, safe='')}/{quoted_path}"


def link_inline_source_code_refs(text: str, repo_url: str, branch: str, repo_root: Path) -> tuple[str, int]:
    """Link inline source-code code spans outside preformatted blocks."""
    parts = PRE_BLOCK_RE.split(text)
    replacements = 0

    for index, part in enumerate(parts):
        if index % 2 == 1:
            continue

        def replace(match: re.Match[str]) -> str:
            nonlocal replacements
            path_text = html.unescape(match.group(1))
            url = source_url(path_text, repo_url, branch, repo_root)
            if url is None:
                return match.group(0)
            replacements += 1
            label = html.escape(path_text, quote=False)
            return f'<a href="{html.escape(url, quote=True)}"><code>{label}</code></a>'

        parts[index] = SOURCE_CODE_RE.sub(replace, part)

    return "".join(parts), replacements


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("site_dir", type=Path, help="MkDocs output directory to post-process")
    parser.add_argument(
        "--repo-url",
        default=os.environ.get("SOURCE_LINK_REPO_URL", "https://github.com/gjbex/Rust-good-bad-ugly"),
        help="GitHub repository URL, default: SOURCE_LINK_REPO_URL or this repository",
    )
    parser.add_argument(
        "--branch",
        default=os.environ.get("SOURCE_LINK_BRANCH", "main"),
        help="Git branch to link to, default: SOURCE_LINK_BRANCH or main",
    )
    args = parser.parse_args()

    repo_root = Path.cwd()
    site_dir = args.site_dir.resolve()
    total_replacements = 0
    changed_files = 0

    for html_file in sorted(site_dir.rglob("*.html")):
        original = html_file.read_text(encoding="utf-8")
        updated, replacements = link_inline_source_code_refs(
            original,
            repo_url=args.repo_url,
            branch=args.branch,
            repo_root=repo_root,
        )
        if replacements:
            html_file.write_text(updated, encoding="utf-8")
            total_replacements += replacements
            changed_files += 1

    print(
        f"Linked {total_replacements} source-code reference(s) "
        f"in {changed_files} generated HTML file(s)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
