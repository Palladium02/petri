#![feature(new_range_api)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![allow(unused)]
mod analysis;
mod args;
mod export;
mod grammar;
mod repl;
mod traits;

use std::fs;

use clap::Parser as ArgParser;

use crate::{
    analysis::analyzer::Analyzer,
    args::{Args, Command},
    export::graphviz::Graphviz,
    grammar::{lexer::Lexer, parser::Parser},
    repl::repl::Repl,
};

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.command.input()).expect("Failed to read input");
    let ast = Parser::new(Lexer::new(&input))
        .parse()
        .expect("Failed to parse");
    let (model, report) = Analyzer::analyze(&ast);

    println!("{}", report.to_string(&input));

    if report.has_error() {
        return;
    }

    match args.command {
        Command::Export { output, engine, .. } => Graphviz::export(&model, output, &engine),
        Command::Interactive { input } => {
            let mut repl = Repl::new(&model);
            repl.run()
        }
    }
}
