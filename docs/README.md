The Rust programming language has gained quite some attention as a systems
programming language with strong safety guarantees.  What are its strong
points, its weak points?  Is it a practical language for scientific computing
and data analysis?  Should you learn it, use it?

This training tries to give you some insight into the language and how it
compares to other programming languages used in scientific computing, such as
C, C++, Python, Julia, R, and Fortran, so that you can answer these questions
for yourself.


## Learning outcomes

When you complete this training you will

  * know how to create, build, and run small Rust projects;
  * write scalar numerical expressions with explicit Rust types;
  * know the main control-flow constructs;
  * understand the basics of ownership, borrowing, and mutation;
  * model data with structs and methods;
  * use traits for reusable abstractions;
  * process collections, iterators, and structured text data;
  * handle missing values and recoverable errors with `Option` and `Result`;
  * organize Rust projects into binaries, libraries, modules, and tests;
  * use random-number generation in reproducible runs;
  * use Rayon for data-parallel computations;
  * relate Rust's ecosystem to scientific-computing workflows.


## Schedule

Total duration: 4 hours.

  | Subject                                      | Duration |
  |----------------------------------------------|----------|
  | introduction and motivation                  | 10 min.  |
  | project workflow and scalar computation      | 35 min.  |
  | control flow and program structure           | 25 min.  |
  | ownership, borrowing, and mutation           | 45 min.  |
  | coffee break                                 | 10 min.  |
  | structs, traits, and iterators               | 45 min.  |
  | error handling, testing, and reproducibility | 30 min.  |
  | parallelism and integrated examples          | 30 min.  |
  | Rust ecosystem                               | 20 min.  |
  | wrap up                                      | 10 min.  |


## Training materials

The [learning modules](learning-modules/) are available as a website.

The [slide deck](slides/rust-good-bad-ugly.html) is available as a Quarto
RevealJS presentation.

The source code, slide sources, and learning-module Markdown files are
available in the [GitHub repository](https://github.com/gjbex/Rust-good-bad-ugly).


## Target audience

This training is for you if you want to learn enough Rust to judge whether it
could work for scientific computing, technical software, data-processing tools,
or command-line applications.


## Prerequisites

You will need experience programming in some programming language such as
Python, Julia, R, C, C++, or Fortran.  This is not a training that teaches you
how to program.

If you plan to use Rust in a Linux or HPC environment, you should be familiar
with these as well.

More concretely, participants should already be comfortable with the following:

* variables, expressions, control flow, and writing functions in some other
  programming language;
* basic data structures such as arrays/lists, maps/dictionaries, or sets;
* reading short programs and understanding how data flows through them;
* basic problem decomposition into helper functions or modules;
* compiling or running programs from the command line at a basic level;
* working in a shell environment well enough to edit files, run commands, and
  inspect output;
* reading compiler or interpreter diagnostics well enough to locate the part of
  a program that needs attention.

You do not need prior experience with Rust itself, ownership and borrowing,
traits, Cargo, Rayon, or Rust's scientific-computing ecosystem. Those are part
of the training itself.


### Quick self-assessment

If you can do most of the tasks below in some programming language, you are
likely ready for this training.

* write a function that computes the average of a list of numbers;
* loop over a collection and compute a derived result such as a sum or count;
* use `if`/`else` or a `switch`/`match`-like construct to classify values into
  cases;
* split a program into a few helper functions or files;
* read data from a text file and print a simple summary;
* handle a missing value or invalid input without crashing the whole program;
* read a short program and explain what it does;
* compile or run a small program from the command line;
* make a small change to an existing program and run it again.

If several of these items still feel difficult, the training will probably move
too fast. In that case, it is better to first take a short introductory
programming course.


### Software and access requirements

To follow hands-on, you need a computer with a Linux-style terminal and a Rust
development environment installed. The recommended setup uses `rustup` for the
Rust toolchain and `cargo` for building and running the examples.

Some examples use Python helper scripts for visualization, and the learning
module website is built with MkDocs. The repository contains an
`environment.yml` file for creating the Python environment with `mamba`.

See the repository's `SETUP.md` file for installation and verification
commands.


## Level

* Introductory: 40 %
* Intermediate: 40 %
* Advanced: 20 %


## Trainer(s)

  * Geert Jan Bex ([geertjan.bex@uhasselt.be](mailto:geertjan.bex@uhasselt.be))
