# The Pitusya Programming Language

<p align="center">
  <img src="dont-readme/PITUSYA.svg" alt="(=^･ω･^=)"/>
</p>


## Overview
The language with no superpowers (no standard library, only one type - a 64 bit width floating point number).
Edit: Standard library has beed added 😎

## Syntax
```pitusya
fn slow_inverse_square_root(x) {
    ret 1 / x * x
}
fn loops_are_working() {
    let a = 0.0
    while a < 100 {
        a = a + 1
        if a == 44 {
            ret a
        }
    }
    ret 0
}
fn main() {
    ret loops_are_working()
}
```

A bit complicated example:
```pitusya
extern print(n)

fn complex(a, b, c) {
    ret a * b / (c * a) + (c / 8) * (a * a)
}
fn main() {
    let a = 11
    let b = 15
    let c = a = b = 10000
    print(complex(a, b, c))
    ret 0
}
```

# Installation
## Prerequisites 
1. LLVM-16 and Clang installed
2. Rust™up installed

## Manual building
Manual building is more prefereable since intalls the latest version
```shell
$ git clone https://github.com/Jujumba/pitusya
$ cargo install --path pitusya
# Great! Pitusya is installed now 
```

## Via Cargo
```shell
$ cargo install pitusya
```