# C++ problems Rust avoids

This directory contains small C++ programs that deliberately demonstrate
problems Rust is designed to prevent or make explicit in safe code.  The
examples are for training and diagnostic demonstrations.  Several programs
execute undefined behavior, so do not use them as templates for real C++ code.


## What is it?

1. `use_after_free.cpp`: deletes a heap object and then reads through the old
   pointer.
1. `dangling_pointer.cpp`: returns the address of a local stack variable after
   the variable has gone out of scope.
1. `double_free.cpp`: deletes the same heap allocation twice.
1. `data_race.cpp`: updates a shared counter from multiple threads without
   synchronization.
1. `buffer_overflow.cpp`: writes past the end of a fixed-size stack array.
1. `uninitialized_read.cpp`: reads an automatic variable before assigning a
   value to it.
1. `memory_leak.cpp`: allocates heap memory and intentionally loses ownership
   without freeing it.
1. `double_promotion.cpp`: shows how a `float` expression is promoted to
   `double` when combined with a double-precision literal.


## How to build?

Configure and build all examples with CMake:

```bash
cmake -S . -B build
cmake --build build
```

The executables are written to the build directory.


## Diagnostics

Many of these problems are easiest to see with sanitizers enabled.  Address and
undefined behavior sanitizers can be enabled for most examples:

```bash
cmake -S . -B build-asan -DENABLE_SANITIZERS=ON
cmake --build build-asan
```

ThreadSanitizer is the appropriate diagnostic tool for the data race example:

```bash
cmake -S . -B build-tsan -DENABLE_THREAD_SANITIZER=ON
cmake --build build-tsan
```

Run one executable at a time.  Some examples may terminate immediately when a
sanitizer detects the bug.
