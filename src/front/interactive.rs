use js::run::exec::Executor;
use js::run::jit::JITCompiler;
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
		debug!("Running interactive mode");
		print!("> ");
		let mut engine : JITCompiler = Executor::new();
		let mut input = stdin();
		loop {
			let line = input.read_line().unwrap();
			debug!("Now parsing line {}", line);
			let line_bytes = line.as_bytes();
			debug!("Now lexing...");
			let mut lexer = Lexer::new(BufferedReader::new(BufReader::new(line_bytes)));
			lexer.lex().unwrap();
			let tokens = lexer.tokens;
			debug!("Lexed into tokens: {}", tokens);
			debug!("Now parsing...");
			let expr = Parser::new(tokens).parse_all().unwrap();
			debug!("Parsed into expression: {}", expr);
			debug!("Now executing with LibJIT backend...");
			let compiled = engine.compile(&expr);
			match engine.run(&compiled) {
				Ok(v) =>
					println!("{}", v),
				Err(v) =>
					println!("Failed with {}", v)
			}
			print!("> ");
		}
	}
}