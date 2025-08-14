# Ready-Set-Boole

## Overview

**Ready-Set-Boole** is a project written in **Rust** that explores a wide range of Boolean algebra concepts and operations. This project serves as a comprehensive exercise to deepen the understanding of both Rust programming and logical computations, from basic bitwise arithmetic to advanced topics like SAT solvers and set theory evaluation.

The project is modular, with each exercise focusing on a specific concept. It includes a robust testing suite to ensure correctness and a central `Formula` engine that powers many of the logical exercises.

## Features

The project is divided into several exercises, each building upon previous concepts:

### Core Logic and Arithmetic
-   **ex00: Bitwise Adder**: Implements a binary adder using only bitwise operations in an iterative fashion.
    -   File: [`adder.rs`](ready-set-boole/src/ex00/adder.rs)
-   **ex01: Bitwise Multiplier**: Implements a binary multiplier using the bitwise adder and shift operations.
    -   File: [`multiplier.rs`](ready-set-boole/src/ex01/multiplier.rs)
-   **ex02: Gray Code**: Converts an integer to its Gray code equivalent using bitwise XOR.
    -   File: [`gray_code.rs`](ready-set-boole/src/ex02/gray_code.rs)

### Boolean Formula Engine
A powerful formula engine is implemented in `extra/formula.rs`, supporting parsing, evaluation, and transformation of logical expressions.

-   **ex03: Formula Evaluation**: Evaluates Boolean formulas expressed in Reverse Polish Notation (RPN).
    -   File: [`eval_formula.rs`](ready-set-boole/src/ex03/eval_formula.rs)
-   **ex04: Truth Table Generator**: Generates and prints a complete truth table for any given Boolean formula.
    -   File: [`truth_table.rs`](ready-set-boole/src/ex04/truth_table.rs)
-   **ex05: Negation Normal Form (NNF)**: Converts a formula into its NNF, pushing negations inward.
    -   File: [`nnf.rs`](ready-set-boole/src/ex05/nnf.rs)
-   **ex06: Conjunctive Normal Form (CNF)**: Converts a formula into its CNF, essential for SAT solvers.
    -   File: [`cnf.rs`](ready-set-boole/src/ex06/cnf.rs)
-   **ex07: SAT Solver**: Determines if a Boolean formula is satisfiable by testing all possible variable assignments.
    -   File: [`sat.rs`](ready-set-boole/src/ex07/sat.rs)

### Set Theory and Advanced Applications
-   **ex08: Powerset**: Generates the powerset (the set of all subsets) of a given set of integers.
    -   File: [`powerset.rs`](ready-set-boole/src/ex08/powerset.rs)
-   **ex09: Set Evaluation**: Evaluates a Boolean formula where variables represent sets, performing set operations (union, intersection, complement).
    -   File: [`set_evaluation.rs`](ready-set-boole/src/ex09/set_evaluation.rs)
-   **ex10: Z-order Curve Mapping**: Maps 2D coordinates (`u16`, `u16`) to a single `f64` value using a Z-order curve (Morton code), preserving locality.
    -   File: [`curve.rs`](ready-set-boole/src/ex10/curve.rs)
-   **ex11: Inverse Z-order Curve**: Implements the inverse function to decode a `f64` value back into its original 2D coordinates.
    -   File: [`inverse_function.rs`](ready-set-boole/src/ex11/inverse_function.rs)

### Unit Testing
-   Comprehensive unit tests for each module ensure correctness and reliability across all functionalities.
    -   Example: [`ex07/test.rs`](ready-set-boole/src/ex07/test.rs), [`ex11/test.rs`](ready-set-boole/src/ex11/test.rs)

## Code Structure

### Modules
-   **`ex00` to `ex11`**: Each directory contains a specific, self-contained exercise.
-   **`extra::formula`**: A central, reusable module that defines the `Formula` enum and its associated methods for parsing, evaluation, and transformation. This is the core of exercises `ex03` through `ex09`.

### Binaries
-   Each exercise can be run as a separate binary, allowing for focused testing and demonstration. The `Cargo.toml` is configured to build each `main.rs` file.

## How to Run

1.  Clone the repository:
    ```sh
    git clone https://github.com/xmatute-/ready-set-boole.git
    cd ready-set-boole/ready-set-boole
    ```
2.  Run tests to validate all functionalities:
    ```sh
    cargo test
    ```
3.  Execute an individual exercise's binary:
    ```sh
    # Example for SAT solver (ex07)
    cargo run --bin ex07 "AB|A!&"

    # Example for Powerset (ex08)
    cargo run --bin ex08 "1 2 3"
    ```

## Reflections

Working on **Ready-Set-Boole** has been an exciting and challenging experience. As my first medium-sized project in Rust, it has pushed me to learn and apply Rust's unique features while exploring the mathematical world of Boolean algebra. Completing this project has provided valuable insights into both programming and problem-solving, from low-level bitwise logic to higher-level abstract algebraic structures.

## Acknowledgments

Special thanks to the **Rust community** for providing excellent documentation, resources, and support. This project has been a rewarding journey into both programming and mathematics.