#![feature(globs)]
#![crate_type = "bin"]
extern crate js;
use js::lexer::Lexer;
use js::parser::Parser;
use js::exec::{Executor, Interpreter};
use std::io;

fn main() {
	let mut engine : Interpreter = Executor::new();
	print!("> ");
	for line in io::stdin().lines() {
		match line {
			Ok(line) => {
				let tokens = Lexer::<io::BufferedReader<io::BufReader>>::lex_str(line.as_slice());
				let expr = Parser::new(tokens).parse_all().unwrap();
				let result = engine.run(&expr);
				match result {
					Ok(v) => print!("{}", v.borrow()),
					Err(v) => print!("Error: {}", v.borrow())
				}
				print!("\n> ");
			},
			Err(err) => {
				fail!("{}", err);
			}
		}
	}
}