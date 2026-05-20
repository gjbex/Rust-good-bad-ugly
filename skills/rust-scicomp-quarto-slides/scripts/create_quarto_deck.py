#!/usr/bin/env python3
"""Draft Quarto revealjs slides from a Rust teaching module Markdown file."""

from __future__ import annotations

import argparse
import re
import textwrap
from dataclasses import dataclass, field
from pathlib import Path


FENCE_RE = re.compile(r"^```(\w+)?\s*$")
HEADING_RE = re.compile(r"^(#{1,3})\s+(.+?)\s*$")
LIST_RE = re.compile(r"^\s*(?:[-*]|\d+\.)\s+(.+?)\s*$")
COMMAND_RE = re.compile(r"^\s*(cd|cargo|python3?|mamba|conda|hyperfine)\b")


@dataclass
class CodeBlock:
    language: str
    lines: list[str]


@dataclass
class Section:
    title: str
    level: int
    lines: list[str] = field(default_factory=list)
    code_blocks: list[CodeBlock] = field(default_factory=list)


def slug_to_title(path: Path) -> str:
    return path.stem.replace("-", " ").title()


def parse_markdown(path: Path) -> tuple[str, list[Section]]:
    title = slug_to_title(path)
    sections: list[Section] = []
    current: Section | None = None
    in_fence = False
    fence_lang = ""
    fence_lines: list[str] = []

    for raw_line in path.read_text(encoding="utf-8").splitlines():
        fence = FENCE_RE.match(raw_line)
        if fence:
            if in_fence:
                if current is not None:
                    current.code_blocks.append(CodeBlock(fence_lang, fence_lines))
                in_fence = False
                fence_lang = ""
                fence_lines = []
            else:
                in_fence = True
                fence_lang = fence.group(1) or ""
                fence_lines = []
            continue

        if in_fence:
            fence_lines.append(raw_line)
            continue

        heading = HEADING_RE.match(raw_line)
        if heading:
            level = len(heading.group(1))
            heading_title = heading.group(2)
            if level == 1:
                title = heading_title
                continue
            if level == 2:
                current = Section(heading_title, level)
                sections.append(current)
                continue

        if current is not None:
            current.lines.append(raw_line)

    return title, sections


def clean_inline(text: str) -> str:
    text = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", text)
    text = re.sub(r"\s+", " ", text).strip()
    return text.rstrip(".")


def first_sentences(lines: list[str], limit: int) -> list[str]:
    text = " ".join(line.strip() for line in lines if line.strip())
    text = re.sub(r"\s+", " ", text)
    candidates = re.split(r"(?<=[.!?])\s+", text)
    return [clean_inline(sentence) for sentence in candidates if sentence][:limit]


def list_items(lines: list[str], limit: int) -> list[str]:
    items: list[str] = []
    current: list[str] = []

    def flush_current() -> None:
        if current:
            items.append(clean_inline(" ".join(current)))
            current.clear()

    for line in lines:
        match = LIST_RE.match(line)
        if match:
            flush_current()
            current.append(match.group(1))
            if len(items) >= limit:
                break
            continue
        if current and line.startswith((" ", "\t")) and line.strip():
            current.append(line.strip())
            continue
        flush_current()
        if len(items) >= limit:
            break
    flush_current()
    return items[:limit]


def command_blocks(section: Section) -> list[CodeBlock]:
    blocks: list[CodeBlock] = []
    for block in section.code_blocks:
        if block.language in {"bash", "sh", "shell", "console"}:
            blocks.append(block)
        elif any(COMMAND_RE.match(line) for line in block.lines):
            blocks.append(CodeBlock("bash", block.lines))
    return blocks


def rust_blocks(section: Section, max_lines: int) -> list[CodeBlock]:
    blocks: list[CodeBlock] = []
    for block in section.code_blocks:
        if block.language == "rust" and len(block.lines) <= max_lines:
            blocks.append(block)
    return blocks


def source_paths(lines: list[str]) -> list[str]:
    paths: list[str] = []
    for line in lines:
        for path in re.findall(r"`(source-code/[^`]+|learning-modules/[^`]+|slides-source/[^`]+)`", line):
            if path not in paths:
                paths.append(path)
    return paths


def runnable_paths(paths: list[str]) -> list[str]:
    return [path for path in paths if path.startswith("source-code/")]


def yaml_header(title: str, subtitle: str, theme: str) -> str:
    return textwrap.dedent(
        f"""\
        ---
        title: "{title}"
        subtitle: "{subtitle}"
        format:
          revealjs:
            theme: {theme}
            slide-number: true
            chalkboard: true
            preview-links: auto
            code-line-numbers: false
        execute:
          echo: true
          eval: false
        ---
        """
    )


def emit_notes(lines: list[str]) -> list[str]:
    return [
        "",
        "::: notes",
        *lines,
        ":::",
        "",
    ]


def emit_code(block: CodeBlock) -> list[str]:
    language = block.language or "text"
    return [f"```{language}", *block.lines, "```", ""]


def build_deck(
    title: str,
    sections: list[Section],
    subtitle: str,
    theme: str,
    max_slides: int,
    max_code_lines: int,
) -> str:
    output: list[str] = [yaml_header(title, subtitle, theme).rstrip(), ""]
    output.extend(
        [
            "## Teaching Arc",
            "",
            "- Establish the concept",
            "- Connect it to scientific-computing practice",
            "- Switch to terminal or source when the example gets long",
            "- Leave room for questions and live edits",
            "",
            *emit_notes(
                [
                    "This deck is a generated draft. Tighten it before teaching:",
                    "remove reading-style prose, split dense slides, and add deliberate live-coding cues.",
                ]
            ),
        ]
    )

    slide_count = 1
    for section in sections:
        if slide_count >= max_slides:
            break

        items = list_items(section.lines, 5)
        sentences = first_sentences(section.lines, 3)
        commands = command_blocks(section)
        rust = rust_blocks(section, max_code_lines)
        paths = source_paths(section.lines)
        runnable = runnable_paths(paths)

        if section.title.lower() == "prerequisites":
            heading = "Prerequisite Reminder"
        elif "suggested hands-on" in section.title.lower():
            heading = "Terminal Or Live Coding"
        else:
            heading = section.title

        output.extend([f"## {heading}", ""])

        bullets = items or sentences
        if not bullets and paths:
            bullets = [f"Inspect `{path}`" for path in paths[:4]]
        if not bullets:
            bullets = ["Connect this concept to the example code"]

        for bullet in bullets[:5]:
            output.append(f"- {bullet}")
        output.append("")

        if commands:
            output.extend(emit_code(commands[0]))
        elif rust:
            output.extend(emit_code(rust[0]))
        elif runnable:
            output.append("```bash")
            output.append(f"cd {runnable[0]}")
            output.append("cargo run")
            output.append("```")
            output.append("")

        notes = [f"Source section: {section.title}."]
        if paths:
            notes.append("Relevant paths: " + ", ".join(f"`{path}`" for path in paths[:4]) + ".")
        if section.code_blocks and not commands and not rust:
            notes.append("Long code exists in the source module; prefer showing it in the editor or terminal.")
        output.extend(emit_notes(notes))
        slide_count += 1

    return "\n".join(output).rstrip() + "\n"


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Create a first-pass Quarto revealjs deck from a Rust teaching module."
    )
    parser.add_argument(
        "module",
        type=Path,
        help="Input Markdown module, usually learning-modules/*.md",
    )
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        help="Output .qmd path. Defaults to slides-source/<module-stem>.qmd",
    )
    parser.add_argument(
        "--subtitle",
        default="Rust for scientific computing",
        help="Deck subtitle",
    )
    parser.add_argument("--theme", default="simple", help="Revealjs theme")
    parser.add_argument("--max-slides", type=int, default=24, help="Maximum generated slides")
    parser.add_argument(
        "--max-code-lines",
        type=int,
        default=10,
        help="Maximum Rust lines to include directly on a slide",
    )
    args = parser.parse_args()

    if not args.module.exists():
        raise SystemExit(f"module not found: {args.module}")

    output = args.output or Path("slides-source") / f"{args.module.stem}.qmd"
    title, sections = parse_markdown(args.module)
    deck = build_deck(title, sections, args.subtitle, args.theme, args.max_slides, args.max_code_lines)

    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(deck, encoding="utf-8")
    print(output)


if __name__ == "__main__":
    main()
