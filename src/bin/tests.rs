#![feature(globs)]
#![crate_type = "bin"]
extern crate script;
extern crate test;
extern crate collections;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use script::js::value::Value;
use std::path::posix::Path;
use std::io::fs::{File, walk_dir};
use std::io::BufferedReader;
use std::task::{task};
fn main() {
	for file in walk_dir(&Path::new("../tests/test/suite")).unwrap() {
		let file_name = file.to_c_str().as_str().unwrap().to_owned();
		if file_name.ends_with(".js") {
			task().named(file_name.clone()).spawn(proc() {
				let mut lexer = Lexer::new();
				lexer.lex(BufferedReader::new(File::open(&file).unwrap())).unwrap();
				let mut parser = Parser::new(lexer.tokens);
				let parsed = match parser.parse_all() {
					Ok(v) => v,
					Err(v) => fail!("Could not parse '{}' due to {}", file_name, v)
				};
				let mut engine : ~Interpreter = Executor::new();
				let result : Result<Value, Value> = engine.run(parsed);
				match result {
					Ok(v) => println!("{}: {}", file_name, v.borrow()),
					Err(v) => fail!("'{}' failed with {}", file_name, v.borrow())
				}
			})
		}
	}
}