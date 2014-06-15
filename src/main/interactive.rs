use back::{JitCompiler, JitExecutor};
use front::run::compiler::Compiler;
use front::run::executor::Executor;
use jit::Context;
use syntax::Lexer;
use syntax::Parser;
use std::io::stdio::{stdin, StdReader};
use std::io::{BufReader, BufferedReader};
/// An interactive command-line mode
pub struct Interactive<'a> {
	context: Context<'a>,
	/// The execution engine to run the expressions on
	pub executor: JitExecutor,
	/// The standard input stream to read from
	pub input: BufferedReader<StdReader>
}
impl<'a> Interactive<'a> {
	/// Create a new interactive mode info
	pub fn new() -> Interactive {
		Interactive {
			context: Context::new(),
			executor: JitExecutor::new(),
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
			debug!("Now compiling");
			let compiler = JitCompiler::new(&self.context);
			let compiled = compiler.compile(&expr);
			debug!("Now executing");
			match self.executor.execute(&compiled) {
				Ok(v) =>
					println!("{}", v),
				Err(v) =>
					println!("Failed with {}", v)
			}
			print!("> ");
		}
	}
}