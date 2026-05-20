# Julia set C++

This is a C++ implementation of the Julia set that mirrors
`julia-set-baseline`. It uses:

* a custom 2D array implementation;
* serial implementation of the algorithm.


## What is it?

1. `src/main.cpp` contains the command-line interface, Julia set computation,
   and output formatting.
1. `src/matrix.h` contains a simple implementation of a 2D array (matrix)
   that is used to store the results of the Julia set computation.
1. `CMakeLists.txt` is the CMake build configuration for the executable.


## How to use?

Configure and build the application:

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
```

Run the application:

```bash
./build/julia-set-cpp
```

The command-line arguments match `julia-set-baseline`:

```bash
./build/julia-set-cpp --width 800 --height 600 --max-iterations 1000 --c-real=-0.5125 --c-imag=0.5213
```
