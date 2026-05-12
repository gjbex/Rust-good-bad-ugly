# Julia set

This directory contains various implemenations of Rust application that compute the Julia set.


## What is it?

1. `julia-set-baseline`: baseline implementation of the Julia set, using a
   single thread, and a custom matrix implementation.
1. `julia-set-mdarray`: implementation of the Julia set, using a single thread,
   and the `ndarray` crate for matrix operations.
1. `view-plot.py`: Python script to visualize the output of the Julia set
   applications, using `matplotlib`.
