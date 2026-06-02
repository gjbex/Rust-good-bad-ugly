# Repository Instructions

## Scope

These instructions apply to the whole repository. This is a training-material
repository, not a single software package. Treat text, slides, source examples,
generated site assets, and cross-reference maps as one connected teaching
system.

## Repository Map

- `learning-modules/`: Markdown learning modules used by MkDocs.
- `slides-source/`: Quarto revealjs slide sources derived from the learning
  modules.
- `source-code/`: Rust, C++, and Python examples used by the modules and
  slides.
- `FEATURE_MAP.md`: concept-to-source-code map.
- `skills/rust-scicomp-quarto-slides/`: repository-specific skill and helper
  script for creating and maintaining Quarto slides from learning modules.
- `git-refactor-hygiene` skill: use this skill whenever tracked files or
  directories are moved or renamed.
- `docs/README.md`: GitHub Pages landing page source.
- `docs/learning-modules/` and `docs/slides/`: generated publishing output.
- `scripts/link_source_code_refs.py`: post-processes generated MkDocs HTML so
  inline `source-code/...` references link to GitHub.

## Core Consistency Rule

Before changing any learning module, slide, source example, directory name, or
feature-map entry, check what else points to it. A local edit is not complete
until the related teaching material remains consistent.

## Audience And Teaching Intent

This repository teaches Rust to scientists and technical programmers. Optimize
changes for teaching clarity, live demonstration, and scientific-computing
relevance rather than for exhaustive Rust coverage.

- Prefer examples that make ownership, borrowing, types, errors,
  reproducibility, numerical work, and parallelism concrete.
- Avoid clever or overly abstract code when direct code teaches the concept
  better.
- Do not add advanced Rust features unless they support the module's learning
  goal.
- Keep participant-facing text practical: what to run, what to observe, and why
  it matters for scientific computing.

## Module-To-Slide Mapping

Every numbered curriculum module in `mkdocs.yml` should normally have a
matching numbered slide source included from
`slides-source/rust-good-bad-ugly.qmd`.

For example:

```text
learning-modules/error-handling.md
slides-source/08-error-handling.qmd
```

Support pages such as `learning-modules/index.md` and
`learning-modules/learning-module-structure.md` do not need one-to-one slide
files. Non-module slide sections such as `motivation.qmd`, `ecosystem.qmd`,
and `finally.qmd` may exist outside the numbered mapping.

When changing `learning-modules/*.md`:

- Update the matching `slides-source/*.qmd` section when the teaching flow,
  examples, commands, or concept emphasis changes.
- If adding a new module, add it to `mkdocs.yml`, `learning-modules/index.md`,
  `learning-modules/learning-module-structure.md`, and the appropriate
  `slides-source/*.qmd` file or include it from `slides-source/rust-good-bad-ugly.qmd`.
- Use the Rust slide skill in `skills/rust-scicomp-quarto-slides/` for
  first-pass slide creation and then do an instructor-quality edit.
- Check inline `source-code/...` references against real paths.

When changing `slides-source/*.qmd`:

- Keep slides aligned with the corresponding learning module.
- Slides should not duplicate the full learning module. Use them for teaching
  flow, prompts, diagrams, short code fragments, terminal cues, and discussion
  anchors.
- Prefer concise trainee-facing slide text and put trainer-only guidance in
  `::: notes`.
- Keep long Rust listings in source files; slides should usually point to a
  path and a terminal command.
- Preserve the revealjs setup in `slides-source/rust-good-bad-ugly.qmd` and
  `slides-source/_quarto.yml` unless the publishing workflow is being changed.

When changing `source-code/`:

- Update every learning module, slide, README, and `FEATURE_MAP.md` entry that
  names the changed path, command, API, output, or concept.
- If moving or renaming tracked files or directories, use the
  `git-refactor-hygiene` skill. Check `git status --short` first, use
  `git mv <old> <new>` for tracked paths, and verify Git reports renames rather
  than unrelated delete/add churn.
- Keep examples small, explicit, and suitable for live teaching.
- Keep commands in nearby README files accurate.
- Run the exact command shown in the relevant learning module or slide when
  practical, especially for examples used in live demos.
- Do not introduce shared abstractions unless several examples genuinely need
  them for teaching clarity.

## Dependencies

Keep dependencies conservative and purposeful.

- Add a Rust crate only when it teaches a concept, simplifies a real example,
  or reflects normal scientific-computing practice.
- Prefer standard-library examples when they are sufficient for the learning
  goal.
- When adding or changing dependencies, update the relevant `Cargo.toml`,
  `Cargo.lock`, README instructions, learning-module text, slides, and
  `FEATURE_MAP.md` entries.
- Do not add tooling dependencies to the repository unless they are needed for
  the training workflow or validation.

## Demo Stability

Examples should be reliable during live training.

- Prefer deterministic output for commands shown in modules and slides.
- Seed randomness explicitly when reproducibility is part of the lesson or when
  the output is discussed in teaching material.
- Keep file paths, input files, and generated outputs predictable.
- Avoid examples that require long runtimes, network access, unusual hardware,
  or fragile local services unless the module is specifically about that topic.

When changing `FEATURE_MAP.md`:

- Verify each referenced path exists.
- Add new source examples to the map under every relevant concept, not only
  under the directory where the example lives.
- Remove or update stale entries immediately when source examples move or
  change purpose.

## Generated Output

- Prefer editing sources in `learning-modules/` and `slides-source/` over
  editing generated files in `docs/learning-modules/` or `docs/slides/`.
- The GitHub Actions workflow builds MkDocs output into
  `docs/learning-modules/`, renders Quarto slides into `docs/slides/`, and
  commits generated assets on pushes to the main branch.
- Do not manually edit `docs/learning-modules/` or `docs/slides/` unless the
  task is explicitly about generated published assets or verifying the full
  publishing workflow.

## Build And Validation Commands

Run commands from the repository root unless noted otherwise.

- Build the learning-module site:

  ```bash
  mkdocs build --strict
  ```

- Build the published MkDocs output and link source references:

  ```bash
  mkdocs build --strict --site-dir docs/learning-modules
  python3 scripts/link_source_code_refs.py docs/learning-modules
  ```

- Render the full slide deck:

  ```bash
  quarto render slides-source/rust-good-bad-ugly.qmd --to revealjs
  ```

- Preview slides while editing:

  ```bash
  slides-source/preview.sh
  ```

- Check an individual Rust example:

  ```bash
  cargo check
  ```

  Run this inside the affected `source-code/...` Cargo project.

- Run tests for a Rust example when tests exist:

  ```bash
  cargo test
  ```

  Run this inside the affected `source-code/...` Cargo project.

## Link And Reference Checks

Use fast text searches before and after renames or conceptual changes:

```bash
rg "source-code/old-name|old-name|old command"
rg "learning-modules/|slides-source/|FEATURE_MAP"
```

After a tracked move or rename, verify the Git view as well:

```bash
git status --short
git diff --cached --summary
```

For source path checks, remember that `scripts/link_source_code_refs.py` only
links inline code spans in generated HTML when the referenced local
`source-code/...` path exists. Code blocks such as `cd source-code/math` should
remain plain and copyable.

Also check ordinary Markdown links, Quarto includes, and MkDocs navigation when
renaming or adding modules:

```bash
rg "old-module-name|old-file-name|old-heading"
rg "{{< include|nav:|source-code/"
```

## Style Guidance

- Keep Markdown plain and readable in both GitHub and MkDocs.
- Keep learning modules narrative and practical for scientists learning Rust.
- Prefer concrete commands and observable behavior over abstract descriptions.
- Keep Rust examples in modern, idiomatic Rust while remaining
  beginner-readable.
- Use clear, descriptive names for variables, functions, types, modules, and
  files. Prefer names that help participants follow the code when it is shown
  on screen.
- Keep documentation and comments intentionally light in example code. Add them
  when they clarify non-obvious teaching points, but avoid cluttering files
  that are used for live training.
- Preserve existing naming and numbering conventions for modules and slides.
- Avoid unrelated formatting churn across generated files, slides, and
  learning modules.

## Pull Request Expectations

For changes that touch more than one layer, describe the consistency work in
the PR summary. Mention which learning modules, slides, source examples, and
feature-map sections were checked, and list the validation commands that were
run.

Before finishing a cross-cutting change, check:

- Affected learning modules were updated.
- Matching numbered slide files were updated or intentionally left unchanged.
- `slides-source/rust-good-bad-ugly.qmd` includes any new numbered slide file.
- `mkdocs.yml` includes any new learning module.
- Source-code paths still exist and source-code references still resolve.
- `FEATURE_MAP.md` reflects new, moved, renamed, or repurposed examples.
- README commands near changed examples are still correct.
- Changed Rust examples pass `cargo check`, and `cargo test` when tests exist.
- Generated files under `docs/learning-modules/` and `docs/slides/` were left
  alone unless regenerating them was intentional.
