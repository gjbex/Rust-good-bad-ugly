# Environment Setup

This document describes how to set up the software environment for the
training material. It assumes that you have access to a Linux-style terminal
and are comfortable running shell commands.

The setup has two parts:

- Rust tooling, installed with `rustup`.
- Python packages for visualization, installed in a `mamba` environment from
  `environment.yml`.
- MkDocs for building the learning-module website.

## Rust Toolchain With `rustup`

The recommended way to install Rust is `rustup`, the Rust toolchain installer
and version manager.

Install it with the command from the official Rust installation page:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. The default installation is appropriate for
this training.

After installation, either restart your shell or source the Cargo environment
file:

```bash
source "$HOME/.cargo/env"
```

Check that the tools are available:

```bash
rustc --version
cargo --version
rustup --version
```

Update the Rust toolchain with:

```bash
rustup update
```

For this repository, the stable toolchain is sufficient:

```bash
rustup default stable
```

If compilation fails because no linker or C compiler is available, install your
Linux distribution's standard build tools. For example, on Debian or Ubuntu:

```bash
sudo apt update
sudo apt install build-essential curl
```

## Checking A Rust Example

After installing Rust, verify the setup with a small example:

```bash
cd source-code/hello-world
cargo run
```

You should see the hello-world output from the program.

To check a larger example without running it:

```bash
cd ../random-numbers
cargo check
```

## Python Environment With `mamba`

Some examples use Python scripts for visualization. The required Python
packages are listed in the top-level `environment.yml` file.

Install `mamba` through Miniforge. The Mamba documentation recommends a fresh
Miniforge installation for new users.

On Linux x86_64, a typical installation is:

```bash
curl -L -O "https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh"
bash Miniforge3-Linux-x86_64.sh
```

Follow the installer prompts. After installation, restart your shell or source
the shell initialization file that the installer updated.

Check that `mamba` is available:

```bash
mamba --version
```

## Create The Training Environment

From the repository root, create the environment from `environment.yml`:

```bash
mamba env create -f environment.yml
```

The environment name is defined in the file:

```yaml
name: rust_good_bad_ugly
```

Activate it with:

```bash
mamba activate rust_good_bad_ugly
```

Check that the Python packages are available:

```bash
python -c "import numpy, pandas, plotly, scipy; print('Python environment OK')"
mkdocs --version
```

## Updating The Environment

If `environment.yml` changes later, update the existing environment with:

```bash
mamba env update -f environment.yml --prune
```

The `--prune` option removes packages that are no longer listed in the
environment file.

## Using The Visualization Scripts

Activate the Python environment before running visualization scripts:

```bash
mamba activate rust_good_bad_ugly
```

For example, visualize Julia set output:

```bash
cd source-code/julia-set/julia-set-baseline
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

Or visualize N-body diagnostics:

```bash
cd source-code/n-body-simulation/rust
cargo run -- --steps 200 --save-evolution evolution.csv
../visualize-evolution.py evolution.csv
```

## Building The Learning-Module Site

The learning modules in `docs/learning-modules` are configured as an MkDocs
site. From the repository root, build the static site with:

```bash
mkdocs build
```

Preview the site locally with:

```bash
mkdocs serve
```

The GitHub Pages workflow builds the Markdown landing page under `/`, this site
under `/learning-modules/`, and the Quarto slide deck under `/slides/`.

## Useful References

- Rust installation: <https://www.rust-lang.org/tools/install/>
- rustup: <https://rustup.rs/>
- MkDocs: <https://www.mkdocs.org/>
- Mamba installation: <https://mamba.readthedocs.io/en/stable/installation/mamba-installation.html>
- Miniforge releases: <https://github.com/conda-forge/miniforge/releases>
