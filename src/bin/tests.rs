#![feature(globs)]
#![crate_type = "bin"]
extern crate script;
extern crate test;
extern crate collections;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use script::ast::{Token, TComment};
use script::js::value::{Value, ResultValue, to_value, from_value};
use std::path::posix::Path;
use std::io::fs::{File, walk_dir};
use std::io::BufferedReader;
fn find_attr(tokens: Vec<Token>, attr:~str) -> Option<~str> {
	for tk in tokens.iter() {
		match tk.data {
			TComment(ref comm) => {
				let comm = comm.clone();
				let desc_loc = comm.find_str(~"@" + attr);
				if desc_loc.is_some() {
					return Some(comm.slice_from(desc_loc.unwrap() + attr.len() + 2).split('\n').next().unwrap().to_owned());
				}
			},
			_ => ()
		}
	}
	return None;
}
fn assert(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let val : bool = from_value(*args.get(0)).unwrap();
	if val {
		Ok(to_value(true))
	} else {
		Err(to_value(false))
	}
}
fn main() {
	for file in walk_dir(&Path::new("../tests/")).unwrap() {
		let file_name = file.to_c_str().as_str().unwrap().to_owned();
		if file_name.ends_with(".js") {
			let mut lexer = Lexer::new(BufferedReader::new(File::open(&file).unwrap()));
			lexer.lex().unwrap();
			let mut parser = Parser::new(lexer.tokens);
			let parsed = match parser.parse_all() {
				Ok(v) => v,
				Err(v) => fail!("{}: {}", file_name, v)
			};
			let mut engine : ~Interpreter = Executor::new();
			engine.set_global("assert".into_maybe_owned(), to_value(assert));
			let result : Result<Value, Value> = engine.run(parsed);
			match result {
				Ok(v) => println!("{}: {}", file_name, v.borrow()),
				Err(v) => fail!("{}: Failed with {}", file_name, 
					v.borrow())
			}
		}
	}
}