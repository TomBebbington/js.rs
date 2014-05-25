#![crate_id = "js_syntax"]
#![comment = "Javascript lexing and parsing"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![experimental]

#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]

#![feature(macro_rules)]

//! This crate provides a complete Javascript parsing library

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