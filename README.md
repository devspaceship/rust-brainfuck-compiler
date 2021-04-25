# Rust Brainfuck Compiler

A Brainfuck compiler written in Rust

## What is Brainfuck ?

According to Wikipedia, [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) is an esoteric programming language created in 1993 by Urban Müller.
Notable for its extreme minimalism, the language consists of only eight simple commands and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers. Brainfuck simply requires one to break commands into microscopic steps.
The language's name is a reference to the slang term brainfuck, which refers to things so complicated or unusual that they exceed the limits of one's understanding.

## How does it work

We can make a compiler by converting the brainfuck code into assembly and then assembling it.
However, it is simpler to transpile the brainfuck into C code and to then compile this C source file with an already existing C compiler, this project is using this method instead.

## Requirements
The only requirement is to have `gcc` installed on your system.

## Usage

```bash
cargo run samples/hello_world.bf [-r/--run]
```

The `-r` or `--run` flag is optional and indicates that you want to run the binary after the compilation is completed.

## Status

- [x] C transpilation method
- [ ] Assembly method
- [ ] Change CLI to clap
- [ ] Add options

