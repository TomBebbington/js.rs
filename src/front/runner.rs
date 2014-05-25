use js::exec::{Executor, Interpreter};
use syntax::Lexer;
use syntax::Parser;
use getopts::Matches;
use std::io::{BufferedReader, File};
use std::path::Path;
/// An command-line script executor
pub struct Runner {
	m: Matches
}
impl Runner {
	/// Create a new interactive mode info
	pub fn new(m: Matches) -> Runner {
		Runner {
			m: m
		}
	}
	/// Run the script
	pub fn run(&self, script: StrBuf) {
		let verbose = self.m.opt_present("v");
		let path = Path::new(script.as_slice());
		if path.exists() {
			let file = File::open(&path).unwrap();
			let mut lexer = Lexer::new(BufferedReader::new(file));
			lexer.lex().unwrap();
			let tokens = lexer.tokens;
			if verbose {
				println!("Lexed into: {}", tokens);
				println!("Parsing");
			}
			let expr = Parser::new(tokens).parse_all().unwrap();
			if verbose {
				println!("Parsed as {}", expr);
				println!("Now running");
			}
			let mut engine:Interpreter = Executor::new();
			let result = engine.run(&expr);
			match result {
				Ok(v) =>
					println!("{}", v.borrow()),
				Err(v) =>
					println!("Failed with {}", v.borrow())
			}
		} else {
			fail!("{} does not exist", script);
		}
	}
}