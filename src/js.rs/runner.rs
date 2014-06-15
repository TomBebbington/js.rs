use js::run::compiler::Compiler;
use js::run::executor::Executor;
use js_jit::JitCompiler;
use js_jit::JitExecutor;
use jit::Context;
use syntax::Lexer;
use syntax::Parser;
use std::io::{BufferedReader, File};
use std::path::Path;
/// An command-line script executor
pub struct Runner {
	/// The path to the script
	pub path: Path
}
impl Runner {
	/// Create a new interactive mode info
	pub fn new(script: String) -> Runner {
		Runner {
			path: Path::new(script.as_slice())
		}
	}
	/// Run the script
	pub fn run(&self) {
		if self.path.exists() {
			let file = File::open(&self.path).unwrap();
			debug!("Now lexing...");
			let mut lexer = Lexer::new(BufferedReader::new(file));
			lexer.lex().unwrap();
			let tokens = lexer.tokens;
			debug!("Now lexed into: {}", tokens);
			debug!("Now parsing...");
			let expr = Parser::new(tokens).parse_all().unwrap();
			debug!("Parsed as {}", expr);
			debug!("Creating JIT Context");
			let context = Context::new();
			debug!("Compiling");
			let compiler = JitCompiler::new(&context);
			let result = compiler.compile(&expr);
			debug!("Now running on JIT backend...");
			match JitExecutor::new().execute(&result) {
				Ok(v) =>
					println!("{}", v),
				Err(v) =>
					println!("Failed with {}", v)
			}
		} else {
			fail!("{} does not exist", self.path.display());
		}
	}
}