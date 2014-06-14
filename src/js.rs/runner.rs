use js::run::exec::execute;
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
	pub fn run(&self, script: String) {
		let path = Path::new(script.as_slice());
		if path.exists() {
			let file = File::open(&path).unwrap();
			debug!("Now lexing...");
			let mut lexer = Lexer::new(BufferedReader::new(file));
			lexer.lex().unwrap();
			let tokens = lexer.tokens;
			debug!("Now lexed into: {}", tokens);
			debug!("Now parsing...");
			let expr = Parser::new(tokens).parse_all().unwrap();
			debug!("Parsed as {}", expr);
			debug!("Now running on JIT backend...");
			match execute(&expr) {
				Ok(v) =>
					println!("{}", v),
				Err(v) =>
					println!("Failed with {}", v)
			}
		} else {
			fail!("{} does not exist", script);
		}
	}
}