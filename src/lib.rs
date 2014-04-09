#![crate_id = "rust_js"]
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

//! A Javascript Parser / Interpreter library
extern crate collections;
extern crate time;
extern crate serialize;
extern crate rand;
/// The Abstract Syntax Tree module
pub mod ast;
/// The lexing module
pub mod lexer;
/// The parsing module
pub mod parser;
/// The execution module
pub mod exec;
/// The javascript core library module
pub mod js {
	/// Contains the Javascript value
	pub mod value;
	/// Functions
	pub mod function;
	/// Contains the Javascript object
	pub mod object;
	/// Contains the Javascript array
	pub mod array;
	/// The global console object
	pub mod console;
	/// The global math object
	pub mod math;
	/// The global JSON object
	pub mod json;
	/// Errors
	pub mod error;
}