# aoc2021
[![Rust](https://github.com/internet-diglett/aoc2021/actions/workflows/rust.yml/badge.svg)](https://github.com/internet-diglett/aoc2021/actions/workflows/rust.yml)
[![Publish in Crate Package Registry](https://github.com/internet-diglett/aoc2021/actions/workflows/crate.yml/badge.svg)](https://github.com/internet-diglett/aoc2021/actions/workflows/crate.yml)

Rust lib and binary for solving Advent of Code 2021 puzzles 

## Getting Started
 
These instructions will give you a copy of the project up and running on 
your local machine for development and testing purposes.

### Prerequisites

Requirements for the software and other tools to build, test and push
- [Rust](https://www.rust-lang.org/tools/install)

### Bulding

```
$ cargo build --release

$ ./target/release/aoc2021 --help
aoc2021 0.1.0
Levon Tarver <ask.levon@gmail.com>
Solves Advent of Code 2021 puzzles

USAGE:
    aoc2021 [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --day <day>      Day of puzzle you want to solve
    -f, --file <file>    Plaintext file containing puzzle input. Defaults to day<N>.txt
```

### Usage

```
$ time ./target/release/aoc2021 -d 1
counting the number of times a depth measurement increases
Part One: 1477
counting the number of times the sum of 3 consecutive measurements increases
Part Two: 1523
./target/release/aoc2021 -d 1  0.00s user 0.00s system 59% cpu 0.002 total
```

Rust is fun. Rust is fast.
