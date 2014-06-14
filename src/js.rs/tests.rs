use js::run::exec::execute_env;
use js::stdlib::value::{ResultValue, Value, to_value, from_value};
use js::stdlib::function::Function;
use syntax::Lexer;
use syntax::Parser;
use syntax::ast::token::{Token, TComment};
use collections::treemap::TreeMap;
use std::io::{BufferedReader, File};
use std::io::fs::walk_dir;
fn find_attrs(tokens: Vec<Token>) -> TreeMap<String, String> {
	let mut map = TreeMap::new();
	for tk in tokens.iter() {
		match tk.data {
			TComment(ref comm) => {
				let current = comm.as_slice();
				if current.starts_with(" @") {
					let space_ind = current.slice_from(1).find(' ').unwrap() + 1;
					let key = current.slice_chars(2, space_ind);
					let value = current.slice_from(space_ind + 1);
					map.insert(key.into_string(), value.into_string());
				}
			},
			_ => ()
		}
	}
	map
}
/// Test against unit tests
pub struct Tests;
impl Tests {
	/// Create a new unit tester
	pub fn new() -> Tests {
		Tests
	}
	/// Run a test
	pub fn run_test(&self, path: Path) {
		fn assert(args:Vec<Value>, _:Value, _:Value, _:Value) -> ResultValue {
			if args.len() < 2 {
				return Err(to_value("'assert' function expects assertion and description arguments"));
			}
			let val : bool = from_value(*args.get(0)).unwrap();
			let desc : Value = *args.get(1);
			if val {
				Ok(desc)
			} else {
				Err(desc)
			}
		}
		let file = path.display();
		debug!("Opened {} for testing", file);
		let mut lexer = Lexer::new(BufferedReader::new(File::open(&path).unwrap()));
		debug!("Lexing");
		lexer.lex().unwrap();
		let tokens = lexer.tokens;
		let attrs = find_attrs(tokens.clone());
		let desc = attrs.find(&"description".into_string()).unwrap();
		debug!("Lexed into: {}", tokens);
		debug!("Parsing");
		let expr = Parser::new(tokens).parse_all().unwrap();
		debug!("Parsed as {}", expr);
		debug!("Now running");
		let env = Value::new_obj(None);
		env.set_field_slice("assert", Function::make(assert, ["condition"]));
		match execute_env(&expr, env) {
			Ok(_) =>
				println!("{}: {}: All tests passed successfully", file, desc),
			Err(v) =>
				println!("{}: {}: Failed with {}", file, desc, v)
		}
	}
	/// Run all the tests in `path`
	pub fn run_tests_in(&self, path: Path) -> () {
		for file in walk_dir(&path).unwrap() {
			if file.is_dir() {
				self.run_tests_in(file);
			} else if file.extension_str() == Some("js") {
				self.run_test(file);
			}
		}
	}
	/// Run the tests mode
	pub fn run(&self) -> () {
		let mut path = Path::new("tests");
		if !path.is_dir() {
			path = Path::new("../tests");
		}
		self.run_tests_in(path);
	}
}