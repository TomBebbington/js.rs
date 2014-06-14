use js::run::exec::Executor;
use js::run::jit::JITCompiler;
use syntax::Lexer;
use syntax::Parser;
use std::io::stdio::{stdin, StdReader};
use std::io::{BufReader, BufferedReader};
/// An interactive command-line mode
pub struct Interactive {
	/// The execution engine to run the expressions on
	pub engine: JITCompiler,
	/// The standard input stream to read from
	pub input: BufferedReader<StdReader>
}
impl Interactive {
	/// Create a new interactive mode info
	pub fn new() -> Interactive {
		Interactive {
			engine: Executor::new(),
			input: stdin()
		}
	}
	/// Run the interactive mode
	pub fn run(&mut self) {
		debug!("Running interactive mode");
		print!("> ");
		loop {
			let line = self.input.read_line().unwrap();
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
			let compiled = self.engine.compile(&expr);
			match self.engine.run(&compiled) {
				Ok(v) =>
					println!("{}", v),
				Err(v) =>
					println!("Failed with {}", v)
			}
			print!("> ");
		}
	}
}