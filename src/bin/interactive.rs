#![feature(globs)]
#![crate_type = "bin"]
extern crate script;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use std::io;

fn main() {
	let mut engine : Interpreter = Executor::new();
	print!("> ");
	for line in io::stdin().lines() {
		match line {
			Ok(line) => {
				let tokens = Lexer::<io::BufferedReader<io::BufReader>>::lex_str(line);
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