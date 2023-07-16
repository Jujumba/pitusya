<p align="center">
  <img src="dont-readme/PITUSYA.svg" alt="(=^･ω･^=)"/>
</p>

# The Pitusya Programming Language

## Overview
The language with no superpowers (no standard library, only one type - a 64 bit width floating point number).

## Syntax
```pitusya
fn slowInverseSquareRoot(x) {
    ret 1 / x * x;
}
fn main() {
    ret slowInverseSquareRoot(4;);
}
```
Mind the second semicolon in arguments passing!

A bit complicated example:
```pitusya
fn complex(a b c) {
    ret a * b / (c * a) + (c / 8) * (a * a);
}
fn main() {
    ret complex(11;15;14;);
}
```

# Installation
## Prerequisites 
1) LLVM-17 and Clang installed (if you are running on LLVM-16 or lower, specify your version in the .env file)
2) Rust™ toolchain

## Manual building
```shell
# Check your LLVM version with: `llvm-config --version`
# And specify it if required
$ git clone https://github.com/Jujumba/pitusya && cd pitusya
$ cargo build --release
# Great! Your Pitusya compiler would be in target/release folder
```

## Via Cargo
```shell
# Requires LLVM-17
$ cargo install pitusya
```