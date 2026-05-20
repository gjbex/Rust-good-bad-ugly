# N-body simulation C++

This is a C++ implementation of the n-body simulation in `../rust`. It uses:

* a `System` class with private particle arrays;
* velocity Verlet time integration;
* optional CSV output for diagnostics and particle states.


## What is it?

1. `src/main.cpp` parses command-line arguments, runs the simulation, and writes
   optional CSV files.
1. `src/system.h` declares the `System` class and particle-state view.
1. `src/system.cpp` implements initialization, acceleration, time integration,
   energy diagnostics, center of mass, and particle-state inspection.
1. `CMakeLists.txt` is the CMake build configuration for the executable.


## How to use?

Configure and build the application:

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
```

Run the simulation with the default parameters:

```bash
./build/n-body-simulation-cpp
```

Save energy and center-of-mass evolution:

```bash
./build/n-body-simulation-cpp --save-evolution evolution.csv
```

Save particle states as CSV, one row per particle per time step:

```bash
./build/n-body-simulation-cpp --save-states states.csv
```

Save both outputs:

```bash
./build/n-body-simulation-cpp --steps 200 --save-evolution evolution.csv --save-states states.csv
```
