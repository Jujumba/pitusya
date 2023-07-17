# The Pitusya Programming Language

<p align="center">
  <img src="dont-readme/PITUSYA.svg" alt="(=^･ω･^=)"/>
</p>


## Overview
The language with no superpowers (no standard library, only one type - a 64 bit width floating point number).

## Syntax
```pitusya
fn slowInverseSquareRoot(x) {
    ret 1 / x * x;
}
fn main() {
    ret slowInverseSquareRoot(4);
}
```

A bit complicated example:
```pitusya
fn complex(a, b, c) {
    ret a * b / (c * a) + (c / 8) * (a * a);
}
fn main() {
    let a = 11;
    let b = 15;
    let c = a = b = 10000;
    ret complex(a, b, c);
}
```

# Installation
## Prerequisites 
1) LLVM-16 and Clang installed (if you are running on LLVM-15 or lower, specify your version in the .env file)
2) Rust™up installed

## Manual building
Manual building is more prefereable since intalls the latest version
```shell
# Check your LLVM version with: `llvm-config --version`
# And specify it if required
$ git clone https://github.com/Jujumba/pitusya
$ cargo install --path pitusya
# Great! Your Pitusya compiler would be in target/release folder
```

## Via Cargo
```shell
# Requires LLVM-16
$ cargo install pitusya
```