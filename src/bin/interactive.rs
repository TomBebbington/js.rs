#![feature(globs)]
#![crate_type = "bin"]
extern crate script;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use std::io;

fn main() {
	let mut engine : ~Interpreter = Executor::new();
	print!("> ");
	for line in io::stdin().lines() {
		match line {
			Ok(line) => {
				let mut lexer = Lexer::new();
				lexer.lex_str(line).v_unwrap();
				let mut parser = Parser::new(lexer.tokens);
				let result_e = parser.parse_all().v_unwrap();
				let result = engine.run(result_e);
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