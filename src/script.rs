#![crate_id = "script"]
#![crate_type = "lib"]
#![comment = "Rust Javascript parsing and interpretation library"]
#![license = "MIT"]
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

//! A Javascript lexing, parsing and execution library
extern crate collections;
extern crate time;
extern crate serialize;
extern crate rand;
extern crate url;
/// Abstract syntax tree for lexing and parsing
pub mod ast;
/// A lexer which transforms a stream into a seqeunce of tokens
pub mod lexer;
/// A parser which transforms a sequence of tokens into Javascript expressions
pub mod parser;
/// An interpreter which runs Javascript expressions
pub mod exec;
/// An implementation of the core Javascript library in Rust
pub mod js {
	/// Javascript values, utility methods and conversion between Javascript values and Rust values
	pub mod value;
	/// The global `Function` object and function value representations
	pub mod function;
	/// The global `Object` object
	pub mod object;
	/// The global `Array` object
	pub mod array;
	/// The global `console` object
	pub mod console;
	/// The global `Math` object
	pub mod math;
	/// The global `JSON` object
	pub mod json;
	/// The global `Number` object with related functions and constants
	pub mod number;
	/// The global `Error` object
	pub mod error;
	/// Contains global methods concerning URIs
	pub mod uri;
	/// The global `String` object
	pub mod string;
}