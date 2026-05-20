---
name: rust-scicomp-quarto-slides
description: Create and maintain Rust scientific-computing Quarto revealjs slides and generated learning-module assets for this repository. Use together with the general quarto-training-slides approach when converting learning-modules/*.md into Rust-focused trainee-facing slides, maintaining slides-source/*.qmd, or keeping the GitHub Pages MkDocs/Quarto workflow aligned.
metadata:
  short-description: Convert Rust modules into teaching slides
---

# Rust Scientific-Computing Quarto Slides

Use this skill for the Rust-specific parts of this repository's training
material. Apply the general `quarto-training-slides` teaching approach for deck
structure, audience boundary, speaker notes, and slide quality.

## Rust Teaching Emphasis

- Prefer "why this matters for scientific computing" over generic Rust syntax
  coverage.
- Use small visible Rust fragments only when they introduce a concept clearly.
- Move longer code to terminal demos, live coding, or source-file inspection.
- Make ownership, borrowing, traits, error handling, reproducibility, and
  parallelism concrete through the repository examples.
- Name scientific-computing concerns directly: arrays, grids, random seeds,
  tolerances, data files, parallel loops, diagnostics, and reproducibility.

## Repository Layout

- Learning-module sources live in `learning-modules/`.
- Slide sources live in `slides-source/`.
- Rust and companion examples live in `source-code/`.
- The published GitHub Pages landing page and generated assets live in `docs/`.
- Generated learning-module HTML is built into `docs/learning-modules/`.
- Generated Quarto slides are built into `docs/slides/`.
- The CI workflow owns generated site updates; keep source edits in
  `learning-modules/` and `slides-source/` unless regenerating output.

## Quarto Defaults

Use revealjs with `execute.eval: false`. The Rust examples in this repository
are Cargo projects rather than small executable Quarto cells, and the
instructor usually runs them in a terminal.

For multi-module decks, use one top-level file such as
`slides-source/rust-good-bad-ugly.qmd` that includes one section file per
learning module. Included section files should start with a level-1 heading
for the revealjs section title slide and level-2 headings for ordinary slides.

## Rust Workflow

1. Read the source module in `learning-modules/` and relevant examples under
   `source-code/`.
2. Identify Rust-specific teaching moments:
   - `cargo check`, `cargo run`, `cargo test`, or benchmark commands
   - compiler diagnostics worth discussing
   - source files that are better shown in an editor than copied onto slides
   - links between Rust language features and scientific-computing practice
3. Draft or update the matching `slides-source/*.qmd` section:
   - keep visible text trainee-facing
   - put trainer-only instructions in `::: notes`
   - keep module title slides clean when the next slide is the Module Arc
   - use terminal cue slides for longer examples
4. Render with Quarto when possible and fix syntax or layout problems.

## Publishing Workflow

For this repository, GitHub Pages is intentionally split:

- `docs/README.md` is the classic Jekyll landing page.
- `mkdocs.yml` builds the learning-module site from `learning-modules/`.
- `.github/workflows/pages.yml` builds MkDocs output into
  `docs/learning-modules/`, renders Quarto slides into `docs/slides/`, and
  commits those generated assets on pushes to the main branch.
- Source-code links in rendered learning modules are handled by
  `scripts/link_source_code_refs.py` after MkDocs has rendered the HTML.

When the published learning-module site should link to repository examples,
post-process generated MkDocs HTML rather than hand-writing GitHub links in the
module sources. Inline code spans such as `source-code/math` can be linked to
GitHub `main`, while code blocks such as `cd source-code/math` should remain
plain and copyable.

## Draft Generator

For a first-pass deck from one Rust Markdown module, run:

```bash
python3 skills/rust-scicomp-quarto-slides/scripts/create_quarto_deck.py \
  learning-modules/scalar-computation-and-numeric-basics.md \
  --output slides-source/02-scalar-computation-and-numeric-basics.qmd
```

Then edit the generated `.qmd`. The script extracts headings, compact bullets,
short Rust code blocks, Cargo/terminal cues, and source paths, but it cannot
replace the instructor design pass.

## Rust Slide Patterns

Terminal cue:

````markdown
## Terminal: Compare Numeric Behavior

```{.bash}
cd source-code/math
cargo run
```

- Compare integer and floating-point division
- Check what happens for negative operands

::: notes
Switch to the terminal. Ask learners what they expect before running the
negative-operand example.
:::
````

Short Rust fragment:

````markdown
## Explicit Conversion

```rust
let count: usize = values.len();
let mean = sum / count as f64;
```

- Rust makes conversion visible
- Choose whether the cast belongs here or at an API boundary
````

## Rust-Specific Quality Checks

- Long Rust snippets should be replaced with a path and demo instruction.
- Every terminal cue should have a concrete command or file path.
- Cargo examples should run from the correct `source-code/` subdirectory.
- Visible slide text should describe what trainees observe or compare, not
  what the trainer should say.
