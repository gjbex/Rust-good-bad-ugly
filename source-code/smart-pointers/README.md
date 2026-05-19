# Smart pointers

This example introduces a recursive binary tree backed by `Box<T>`.  Compared to
`../traits`, it shows how to:

* define a recursive data structure using `Option<Box<Node<T>>>`;
* separate a general binary tree module from binary-search-tree algorithms;
* use crate-private methods to expose mutation only inside the library crate;
* add trait bounds such as `T: Ord` only where algorithms require ordering;
* move owned values into a recursive data structure without requiring `Copy`;
* borrow search keys so lookup works for non-`Copy` types such as `String`;
* implement `Display` for a structural tree dump;
* derive `serde::Serialize` and `serde::Deserialize` for recursive data;
* add unit tests for tree formatting, serialization, and BST behavior.

The `tree` module is intentionally a general binary tree.  The ordering invariant
belongs to the `bst` module, which provides insertion and search algorithms on
top of that structure.


## What is it?

1. `src/main.rs`: main source file for the application.  It builds a random
   binary search tree, prints the tree structure, and compares search results
   with a `BTreeSet`.
1. `src/tree.rs`: module that defines the generic `Tree<T>` and `Node<T>` types,
   their read-only public accessors, crate-private mutable links, `Display`
   implementation, and serde support.
1. `src/bst.rs`: module that implements binary-search-tree insertion and search
   as algorithms over the general tree structure.
1. `src/lib.rs`: library target that exposes the `tree` and `bst` modules.
1. `Cargo.toml`: configuration file for the Rust package manager.  It specifies
   the dependencies on `clap`, `rand`, and `serde`, plus the test dependency on
   `serde_json`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically generated
   when building the application.


## How to use?

Run the application:

```bash
cargo run
```

Choose the number of inserted values and the random seed:

```bash
cargo run -- --num-elements 10 --seed 123
```

Show the command-line help:

```bash
cargo run -- --help
```

Run the tests:

```bash
cargo test
```
