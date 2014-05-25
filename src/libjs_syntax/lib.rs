#![crate_id = "js_syntax"]
#![comment = "Javascript lexing and parsing"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![deny(non_uppercase_statics)]
#![deny(missing_doc)]
#![deny(unnecessary_parens)]
#![deny(unrecognized_lint)]
#![deny(unreachable_code)]
#![deny(unnecessary_allocation)]
#![deny(unnecessary_typecast)]
#![deny(unnecessary_allocation)]
#![deny(uppercase_variables)]
#![deny(non_camel_case_types)]
#![deny(unused_must_use)]

#![feature(macro_rules)]

//! Javascript parsing library

extern crate collections;
pub use lexer::Lexer;
pub use parser::Parser;
/// The Abstract Syntax Trees for Javascript tokens and expressions
pub mod ast {
	/// Constant AST
	pub mod constant;
	/// Expression AST
	pub mod expr;
	/// Operation AST
	pub mod op;
	/// Position AST
	pub mod pos;
	/// Token AST
	pub mod token;
}
/// The lexer, which transforms a string stream into a sequence of tokens
pub mod lexer;
/// The parser, which transforms a sequence of tokens into expressions
pub mod parser;