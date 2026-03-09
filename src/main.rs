#![feature(new_range_api)]
#![allow(unused)]
mod grammar;

use std::fs;

use crate::grammar::{lexer::Lexer, parser::Parser};

fn main() {
    let input = fs::read_to_string("./examples/example1.ptr").unwrap();
    let ast = Parser::new(Lexer::new(&input)).parse();

    println!("{}", ast.is_ok())
    // println!("Hello, world!");
}
