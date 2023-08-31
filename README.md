# Advent of Code - Rust

This repository contains my solutions to the [Advent of Code 2022](https://adventofcode.com/2022) challenges, written in Rust.

# Purpose

I'm using these challenges to learn Rust. This is the first time I've used the language, 
so I'm sure there are many things that could be done better. 
I am wanting to see if Rust would a good option for a project at work for a big data pipeline, 
requiring high performance and low memory usage. So, this is a learning exercise for me.

# Structure

Each day has its own file, with a function for each part of the challenge.
The `main.rs` file is used to run the appropriate day and part based on the command line arguments.

### Inputs

The inputs for each day are stored in the `inputs` directory. Each day has its own file, named `day_#.txt`.

# Running

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Clone

```sh
git clone https://github.com/mir-mirsodikov/advent-of-code-2022
```

## Usage

The program is a command line application. It takes two arguments, the day to run and the part of the challenge to run.

The following command will run the solution for day 1, part 1.
```sh
cargo run -- 1 1
```

## Testing

For each day, there are two parts. Each part is its own function, and each function has its own test.
Along with the tests for the parts, there are also tests for the helpers used by the parts.

```sh
cargo test
```

## Building and Running

```sh
cargo build --release
./target/release/advent-of-code-2022 1 1
```



