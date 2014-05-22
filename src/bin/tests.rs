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
				let comm = comm.as_slice();
				for (_, to) in comm.match_indices("// @") {
					let current = comm.slice_from(to);
					let space_ind = current.find(' ').unwrap();
					let key = comm.slice_to(space_ind);
					let value = comm.slice_chars(space_ind, current.find('\n').unwrap());
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
	if val {
		Ok(to_value(true))
	} else {
		Err(to_value(false))
	}
}
fn main() {
	for file in walk_dir(&Path::new("../tests/")).unwrap() {
		if file.is_file() && file.extension_str() == Some("js") {
			let file_str = file.as_str();
			let mut lexer = Lexer::new(BufferedReader::new(File::open(&file).unwrap()));
			lexer.lex().unwrap();
			let mut parser = Parser::new(lexer.tokens);
			let expr = match parser.parse_all() {
				Ok(v) => v,
				Err(v) => fail!("{}: {}", file_str, v)
			};
			let mut engine : Interpreter = Executor::new();
			engine.set_global("assert".into_maybe_owned(), to_value(assert));
			let result : Result<Value, Value> = engine.run(&expr);
			match result {
				Ok(v) => println!("{}: {}", file_str, v.borrow()),
				Err(v) => fail!("{}: Failed with {}", file_str, v.borrow())
			}
		}
	}
}