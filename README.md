# rust-easing
[![Build Status](https://travis-ci.org/orhanbalci/rust-easing.svg?branch=master)](https://travis-ci.org/orhanbalci/rust-easing)
[![Coverage Status](https://coveralls.io/repos/github/orhanbalci/rust-easing/badge.svg?branch=master)](https://coveralls.io/github/orhanbalci/rust-easing?branch=master)
![License](https://img.shields.io/github/license/orhanbalci/rust-easing.svg)
[![Crate Version](https://img.shields.io/crates/v/easer.svg)](https://crates.io/crates/easer)
[![Documentation](https://docs.rs/easer/badge.svg)](https://docs.rs/easer)

Tiny Rust library implementing Robert Penner's easing functions.

# Usage
Add this to your Cargo.toml
``` toml
[dependencies]
easer = "0.2.1"
```
Add this to top of your code file

``` rust
extern crate easer
```
# Example
``` rust
use easer::functions::*;
let mut y: [f64; 100] = [0.0; 100];
for i in 0..100 {
    y[i] = i as f64;
}
println!("Before {:?}", &y[..]);
y.iter_mut().map(|a| *a = Back::ease_in(*a, 0.0, 100.0, 100.0)).count();
println!("After {:?}", &y[..]);
```
