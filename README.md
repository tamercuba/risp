[![Codacy Badge](https://app.codacy.com/project/badge/Grade/566ed174b0c64e27ace59f1a6356b0d4)](https://app.codacy.com/gh/tamercuba/risp/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codacy Badge](https://app.codacy.com/project/badge/Coverage/566ed174b0c64e27ace59f1a6356b0d4)](https://app.codacy.com/gh/tamercuba/risp/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_coverage)

# RISP

## Description

RISP is a personal study project aimed at learning and experimenting with building a Lisp-like interpreter in Rust. This project is not intended for production use and should not be considered stable or reliable for critical applications.

## Modules

### RISP Library
The RISP library is the core component of the project, consisting of:
- **Lexer**: Tokenizes the input strings.
- **Parser**: Converts tokens to `Objects` like described by (Vishal Patil)[https://github.com/vishpat] in his book [Lisp interpreter in Rust](https://vishpat.github.io/lisp-rs/).
- - [As soon as possible](https://github.com/tamercuba/risp/issues/8) ill implement an AST
- **Evaluator**: Evaluates the ASTs and produces results.

### REPL
The REPL (Read-Eval-Print Loop) provides an interactive interface to the RISP interpreter, allowing for direct input and immediate evaluation of RISP expressions.

### Interpreter
Coming soon: An interpreter capable of reading and executing `.risp` files.

## Building the Project

To build and run the project, use the following commands:

- **Run the REPL**:
  ```bash
  cargo repl
  ```

- **Build the REPL**:
  ```bash
  cargo build-repl
  ```

- **Build the RISP library**:
  ```bash
  cargo build-risp
  ```

- **Run all tests**:
  ```bash
  cargo test-lib
  ```

- **Run specific tests**:
  ```bash
  cargo test-match {param}
  ```

## Examples

TODO

---