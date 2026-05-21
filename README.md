# Rust, the good, the bad, and the ugly

Material for a training session on Rust, specifically from the perspective of
scientific computing.  The goals are to:

* Introduce Rust to scientists who are currently not using Rust, but would like
  to assess whether Rust is a good fit for their work.
* Provide a practical introduction to Rust for scientists.

Note that this is not an exhaustive Rust training, it aims to give you a flavor
of what Rust programming is like. Note that this is not an exhaustive Rust
training, it aims to give you a flavor of what Rust programming is like.

Note that this is not an exhaustive Rust training, it aims to give you a flavor
of what Rust programming is like.


## What is it?

1. `source-code`: source code for the training session.
1. `learning-modules`: Markdown learning modules, configured as an MkDocs site.
1. `docs/README.md`: Jekyll landing page source for the published GitHub Pages
   site.
1. `mkdocs.yml`: MkDocs configuration for building the learning-module site.
1. `environment.yml`: conda environment file for the training session, used to
   run visualization scripts and build the MkDocs site.
1. `SETUP.md`: instructions for installing Rust with `rustup` and recreating
   the Python visualization environment with `mamba`.
1. How to contribute: see the [contributing guidelines](CONTRIBUTING.md).
1. License: see the [LICENSE](LICENSE) file.
1. Code of Conduct: see the [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md) file.


## Repository maintenance

This repository includes an `AGENTS.md` file with instructions for maintainers
and AI coding agents. It documents how to keep learning modules, slide sources,
source-code examples, generated site assets, and `FEATURE_MAP.md` consistent.

The `skills/` directory contains repository-specific agent guidance and helper
scripts, including support for creating and maintaining Quarto slides derived
from the learning modules. These files are maintenance tooling and are not
required for participants following the training.
