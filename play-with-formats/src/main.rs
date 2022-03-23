#![allow(unstable_features)]
use std::io::{_print, format_args_nl};

fn main() {
    println!("Hello, world!");
    _print(format_args_nl!(""))
}
