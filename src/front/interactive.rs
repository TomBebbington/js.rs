use js::exec::{Executor, Interpreter};
use syntax::Lexer;
use syntax::Parser;
use getopts::Matches;
use std::io::stdio::stdin;
use std::io::{BufReader, BufferedReader};
/// An interactive command-line mode
pub struct Interactive {
	m: Matches
}
impl Interactive {
	/// Create a new interactive mode info
	pub fn new(m: Matches) -> Interactive {
		Interactive {
			m: m
		}
	}
	/// Run the interactive mode
	pub fn run(&self) {
		print!("> ");
		let mut engine : Interpreter = Executor::new();
		let verbose = self.m.opt_present("v");
		let mut input = stdin();
		loop {
			let line = input.read_line().unwrap();
			let line_bytes = line.as_bytes();
			if verbose {
				println!("Lexing");
			}
			let mut lexer = Lexer::new(BufferedReader::new(BufReader::new(line_bytes)));
			lexer.lex().unwrap();
			let tokens = lexer.tokens;
			if verbose {
				println!("Tokens: {}", tokens);
				println!("Parsing");
			}
			let expr = Parser::new(tokens).parse_all().unwrap();
			if verbose {
				println!("Expression: {}", expr);
			}
			let result = engine.run(&expr);
			match result {
				Ok(v) =>
					println!("{}", v.borrow()),
				Err(v) =>
					println!("Failed with {}", v.borrow())
			}
			print!("> ");
		}
	}
}