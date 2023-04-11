# Brainfuck Rust 🤯🦀

[![CI](https://github.com/k0nze/brainfuck_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/k0nze/brainfuck_rust/actions/workflows/ci.yml)
[![Dependencies](https://deps.rs/repo/github/k0nze/brainfuck_rust/status.svg)](https://deps.rs/repo/github/k0nze/brainfuck_rust)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

To get familiar with Rust as a programming language, I decided to write an interpreter for the programming language [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck).

## The Brainfuck Programming Language

Even though the name of this programming language sounds intimidating, it is reasonably easy to understand. In its essence Brainfuck simulates a Turing machine with 30000 byte cells, a data pointer pointing a the current cell, and a command pointer pointing at the current command to be executed. Using eight different commands, the value of those can be manipulated and printed. Different characters represent those eight commands:

| Character | Command                                                                                      |
|-----------|----------------------------------------------------------------------------------------------|
| `>`		| Increment the data pointer (move right to the next cell).                                    |
| `<`		| Decrement the data pointer (move left to the previous cell).                                 |
| `+`		| Increment current cell value by one.                                                         |
| `-`		| Decrement current cell value by one.                                                         |
| `.`		| Output cell value at which data pointer currently points to.                                 |
| `,`		| Read an input and store it in the cell the data pointer currently points to.                 |
| `[`		| If the current cell is zero move the command pointer forward to the matching `]` .           |
| `]`		| If the current cell is non-zero move the command pointer backward to the matching `[`        |

Before running a Brainfuck program all cells are set to 0 and the data pointer points that the first cell.

### Example Program

```
++[-]+>
```

The Brainfuck program shown above first increments cell 0 to 2 `++`. Then decrements cell 0 until it is 0 again `[-]`. Then increments cell 0 by one `+` and moves the data pointer `>` to the right to point to cell 1.

## Usage

Run hello world example with cargo:
```bash
cargo run -- tests/hello_world.bf
```

Build executable and run:
```bash
cargo build --release --all-features
cd target/release
/brainfuck_rust ../../tests/hello_world.bf
```
