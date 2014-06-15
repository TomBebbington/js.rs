#![crate_id = "github.com/TomBebbington/js.rs#js_syntax:0.1"]
#![comment = "Javascript lexing and parsing"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(
	html_favicon_url = "http://tombebbington.github.io/favicon.png",
	html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![experimental]

#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]

#![feature(macro_rules, globs)]

//! This crate provides a Javascript parsing library with a parser, 
//! a lexer, and Abstract Syntax Tree. The lexer started off based
//! off the Kaleidocope OCaml tutorial, then it evolved off there.
//! 
//! The parser is based on my work on the [`hscript` project from 
//! Haxe](https://github.com/TomBebbington/hscript/blob/master/hscript/Parser.hx), but it's still
//! very premature and some important things are missing for now.

extern crate collections;
pub use lexer::Lexer;
pub use parser::Parser;
/// The Abstract Syntax Trees for Javascript tokens and expressions
pub mod ast {
	/// Constant AST
	pub mod constant;
	/// Expression AST
	pub mod expr;
	/// Typed expression AST
	pub mod typer;
	/// Operations AST
	pub mod op;
	/// Keywords AST
	pub mod keyword;
	/// Punctation AST
	pub mod punc;
	/// Position AST
	pub mod pos;
	/// Token AST
	pub mod token;
	/// Typing AST
	pub mod types;
}
/// The lexer, which transforms a string stream into a sequence of tokens
pub mod lexer;
/// The parser, which transforms a sequence of tokens into expressions
pub mod parser;