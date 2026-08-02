# Training Programs

This training can be delivered as either a four-hour introduction or a
twelve-hour full program. The introduction is also the first session of the
full program, so participants can continue later without repeating material.

Each four-hour session consists of three hours of core teaching, including a
short break, followed by a one-hour optional workshop. No required concepts,
conclusions, or announcements follow the workshop. Participants who prefer to
practice independently can therefore leave after the core teaching without
missing part of the course.

During an optional workshop, participants may follow a guided exercise, ask
questions, or independently explore the provided examples. Subsequent sessions
always start from the repository's reference state and do not depend on
completed exercises.

## Four-Hour Introduction

### Core Teaching: Three Hours

| Time | Content | Modules |
| --- | --- | --- |
| 00:00-00:10 | Motivation, audience, and course map | Introduction |
| 00:10-00:35 | Cargo projects, dependencies, and compiler diagnostics | 1 |
| 00:35-01:05 | Numeric types, conversions, and scientific expressions | 2 |
| 01:05-01:30 | Functions, loops, enums, and `match` | 3 |
| 01:30-01:40 | Break | |
| 01:40-02:25 | Ownership, moves, borrowing, mutation, and slices | 4 |
| 02:25-02:40 | Preview of structs, collections, and explicit errors | 5, 7-8 |
| 02:40-03:00 | Julia-set walkthrough and synthesis | 12 |

The introduction gives participants a working mental model and enough context
to explore Rust projects. Traits, testing, parallelism, multidimensional
arrays, and native interoperability are deliberately deferred to the full
program.

### Optional Workshop: One Hour

The guided path is:

1. Build and modify `source-code/hello-world` or `source-code/hello-clap`.
2. Modify a numerical or control-flow example.
3. Introduce and interpret a compiler error.
4. Change a function from an owned `Vec<f64>` argument to a slice.
5. Experiment with a mutable slice or diagnose a borrowing error.

## Twelve-Hour Full Program

The full program consists of nine hours of core teaching and three optional
workshop hours. Session 1 is the four-hour introduction described above.

### Session 1: Rust Foundations

Use the four-hour introduction without modification:

- Three hours of core teaching.
- One-hour optional workshop.

### Session 2: Idiomatic And Reliable Programs

#### Core Teaching: Three Hours

| Time | Content | Modules |
| --- | --- | --- |
| 00:00-00:05 | Recap and transition from values to domain types | 1-4 |
| 00:05-00:35 | Structs, methods, encapsulation, and generic types | 5 |
| 00:35-01:15 | Traits, trait bounds, conversions, and dispatch | 6 |
| 01:15-01:25 | Break | |
| 01:25-02:05 | Collections, iterators, strings, and text processing | 7 |
| 02:05-02:30 | `Option`, `Result`, error propagation, and `?` | 8 |
| 02:30-02:55 | Libraries, multiple binaries, and scientific unit tests | 9 |
| 02:55-03:00 | Synthesis and workshop handover | 5-9 |

#### Optional Workshop: One Hour

Participants choose one guided track:

- Extend the matrix type with a method and trait implementation.
- Build an iterator pipeline for text or numerical data.
- Add an error path and replace an `expect` with proper propagation.
- Add a numerical unit test with a defensible tolerance.

### Session 3: Scientific Rust Workflows

#### Core Teaching: Three Hours

| Time | Content | Modules |
| --- | --- | --- |
| 00:00-00:15 | Controlled randomness, seeds, and reproducible commands | 10 |
| 00:15-00:40 | Rayon, serial equivalence, and performance measurement | 11 |
| 00:40-01:10 | Julia set and N-body simulation as integrated programs | 12-13 |
| 01:10-01:20 | Break | |
| 01:20-02:10 | `ndarray`, views, broadcasting, stencils, SVD, and HDF5 | 14 |
| 02:10-02:55 | C++ facade, raw FFI, safe wrappers, RAII, and FFTW | 15 |
| 02:55-03:00 | Course conclusions and further study routes | 10-15 |

#### Optional Workshop: One Hour

Participants select one substantial track:

- **Parallelism:** compare serial and Rayon output, then measure the effect of
  different thread counts.
- **Arrays:** modify the heat-diffusion initialization or perform a tested
  stencil refactoring.
- **Reproducibility:** modify and reproduce a Julia-set or N-body
  configuration.
- **Native interoperability:** trace a call through the C++ facade and design a
  small safe wrapper method.
Solutions and expected outputs should be available independently so that the
workshops can also be completed after the training. FFTW, OpenBLAS, and HDF5
activities should be treated as code-reading or instructor-led exercises when
participants have not completed the native-library environment setup.
