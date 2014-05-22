#![crate_type = "bin"]
extern crate script;
extern crate collections;
use script::lexer::Lexer;
use script::parser::Parser;
use script::exec::{Executor, Interpreter};
use script::ast::{Token, TComment};
use script::js::value::{Value, ResultValue, to_value, from_value};
use collections::treemap::TreeMap;
use std::path::posix::Path;
use std::io::fs::{File, walk_dir};
use std::io::BufferedReader;
fn find_attrs(tokens: Vec<Token>) -> TreeMap<~str, ~str> {
	let mut map = TreeMap::new();
	for tk in tokens.iter() {
		match tk.data {
			TComment(ref comm) => {
				let current = comm.as_slice();
				if current.starts_with(" @") {
					let space_ind = current.slice_from(1).find(' ').unwrap() + 1;
					let key = comm.slice_chars(2, space_ind);
					let value = comm.slice_from(space_ind);
					map.insert(key.into_owned(), value.into_owned());
				}
			},
			_ => ()
		}
	}
	map
}
fn assert(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let val : bool = from_value(*args.get(0)).unwrap();
	let desc : Value = *args.get(1);
	if val {
		Ok(desc)
	} else {
		Err(desc)
	}
}
fn main() {
	let mut path = Path::new("tests");
	if !path.is_dir() {
		path = Path::new("../tests");
	}
	if !path.is_dir() {
		fail!("Could not find tests directory");
	}
	for file in walk_dir(&path).unwrap() {
		if file.is_file() && file.extension_str() == Some("js") {
			let file_str = file.as_str().unwrap();
			let mut lexer = Lexer::new(BufferedReader::new(File::open(&file).unwrap()));
			lexer.lex().unwrap();
			let attributes = find_attrs(lexer.tokens.clone());
			let description = match attributes.find(&"description".into_owned()) {
				Some(desc) => desc,
				None => fail!("{} does not have @description metadata", file_str)
			};
			let mut parser = Parser::new(lexer.tokens);
			let expr = match parser.parse_all() {
				Ok(v) => v,
				Err(v) => fail!("{}: {}", file_str, v)
			};
			let mut engine : Interpreter = Executor::new();
			engine.set_global("assert".into_maybe_owned(), to_value(assert));
			let result : Result<Value, Value> = engine.run(&expr);
			match result {
				Ok(_) => println!("All tests passed: {} - {}", file_str, description),
				Err(v) => fail!("Test failed: {} - {}: {}", file_str, description, v.borrow())
			}
		}
	}
}