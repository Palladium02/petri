#![feature(new_range_api)]
#![allow(unused)]
mod grammar;

use std::fs;

use crate::grammar::{analysis::Analyzer, lexer::Lexer, parser::Parser};

fn main() {
    let input = fs::read_to_string("./examples/example1.ptr").unwrap();
    let ast = Parser::new(Lexer::new(&input)).parse();

    match ast {
        Ok(ast) => {
            println!("{:#?}", &ast);
            let report = Analyzer::new(ast).analyze();
            println!("{:#?}", report)
        }
        Err(_) => println!("Whoopsie daisy"),
    }
}
