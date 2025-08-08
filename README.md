# Ready-Set-Boole

## Overview

**Ready-Set-Boole** is a work-in-progress project written in **Rust** that explores Boolean algebra concepts and operations. This is my first medium-sized project in Rust, and it serves as a learning experience to deepen my understanding of both Rust programming and Boolean algebra. The project implements fundamental Boolean operations, Gray code generation, and formula evaluation, showcasing the power and efficiency of Rust for mathematical and logical computations.

The project is modular, with each exercise focusing on a specific aspect of Boolean algebra. It includes unit tests to ensure correctness and reliability, and the code is structured to be extensible for future enhancements.

## Features

### Boolean Operations
- **Adder**:
  - Implements a binary adder using bitwise operations and recursion.
  - Handles edge cases and large numbers.
  - File: [`adder.rs`](ready-set-boole/src/ex00/adder.rs)
- **Multiplier**:
  - Implements a binary multiplier using Rust's wrapping multiplication.
  - Includes tests for zero, one, basic, random, and edge cases.
  - File: [`multiplier.rs`](ready-set-boole/src/ex01/multiplier.rs)

### Gray Code
- **Gray Code Generator**:
  - Generates Gray codes for a given number using bitwise operations.
  - File: [`gray_code.rs`](ready-set-boole/src/ex02/gray_code.rs)

### Formula Evaluation
- **Boolean Formula Evaluator**:
  - Evaluates Boolean formulas using a stack-based approach.
  - Supports operators like `!`, `&`, `|`, `^`, `>`, and `=`.
  - File: [`eval_formula.rs`](ready-set-boole/src/ex03/eval_formula.rs)
- **Formula Evaluator v2**:
  - An alternative implementation of the formula evaluator.
  - File: [`eval_formula_v2.rs`](ready-set-boole/src/ex03/eval_formula_v2.rs)

### Unit Testing
- Comprehensive unit tests for each module:
  - **Adder Tests**: [`test.rs`](ready-set-boole/src/ex00/test.rs)
  - **Multiplier Tests**: [`test.rs`](ready-set-boole/src/ex01/test.rs)
  - **Formula Evaluator Tests**: [`test.rs`](ready-set-boole/src/ex03/test.rs)

## Code Structure

### Modules
- **Adder**: Implements binary addition using recursion.
- **Multiplier**: Handles binary multiplication with edge case testing.
- **Gray Code**: Generates Gray codes using bitwise operations.
- **Formula Evaluator**: Evaluates Boolean formulas with stack-based logic.

### Tests
- Each module includes extensive unit tests to validate functionality and handle edge cases.

### Utilities
- **Stack-Based Logic**: Used in formula evaluation for efficient operator handling.

## Competencies Involved

### Technical Skills
- **Rust Programming**:
  - Learning Rust's syntax, ownership model, and error handling.
  - Using features like pattern matching, recursion, and bitwise operations.
- **Boolean Algebra**:
  - Implementing fundamental operations like addition, multiplication, and formula evaluation.
  - Understanding Gray code generation and its applications.
- **Unit Testing**:
  - Writing comprehensive tests to ensure correctness and reliability.

### Problem-Solving
- **Edge Case Handling**:
  - Managing scenarios like zero, large numbers, and invalid inputs.
- **Optimization**:
  - Using efficient algorithms and Rust's performance-oriented features.

### Personal Growth
- **Learning Rust**:
  - Gaining confidence in a new programming language.
- **Exploring Algebra**:
  - Deepening understanding of Boolean algebra concepts.

## Reflections

Working on **Ready-Set-Boole** has been an exciting and challenging experience. As my first medium-sized project in Rust, it has pushed me to learn and apply Rust's unique features while exploring the mathematical world of Boolean algebra. Although the project is still a work-in-progress, it has already provided valuable insights into both programming and problem-solving. I look forward to completing it and expanding its functionality in the future.

## How to Run

1. Clone the repository:
   ```sh
   git clone https://github.com/xmatute-/ready-set-boole.git
   cd ready-set-boole
   ```
2. Run tests to validate functionality:
   ```sh
   cargo test
   ```
3. Execute individual modules:
   ```sh
   cargo run --bin <module_name>
   ```

## Status

This project is a **work-in-progress**. Current focus areas include:
- Refining the formula evaluator.
- Adding more advanced Boolean operations.
- Improving code documentation and comments.

## Acknowledgments

Special thanks to the **Rust community** for providing resources and support. This project has been a rewarding journey into both programming and mathematics, and I look forward to continuing my exploration of Rust and Boolean algebra.