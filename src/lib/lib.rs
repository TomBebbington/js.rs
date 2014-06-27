#![crate_id = "github.com/TomBebbington/js.rs#js:0.1"]
#![comment = "Javascript parsing, compilation and execution using LibJIT"]
#![license = "MIT"]
#![crate_type = "lib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png",
    html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![experimental]
#![feature(phase, macro_rules, globs)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint,
	unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation,
	uppercase_variables, non_camel_case_types, unused_must_use)]
//! This is a library with seperate modules for Javascript parsing, the Javascript
//! standard library, and Javascript execution through LibJIT
extern crate collections;
extern crate jit;
#[phase(plugin, link)]
extern crate log;
extern crate serialize;
extern crate time;
extern crate url;
/// The backend-defining traits and the Javascript standard library
pub mod front {
	#[macro_escape]
	/// A macro which makes Javascript objects with pretty Rust syntax
	pub mod macro;
	/// Backend-defining traits
	pub mod run {
		/// For compiling Javascript values
		pub mod compiler;
		/// For executing the compiled Javascript values
		pub mod executor;
	}
	/// The Javascript standard library
	pub mod stdlib {
		/// The `Array` global object
		pub mod array;
		/// The `Boolean` global object
		pub mod boolean;
		/// The `console` global object
		pub mod console;
		/// The `Error` global objects
		pub mod error;
		/// The `Function` global object
		pub mod function;
		/// The `JSON` global object
		pub mod json;
		/// The `Math` global object
		pub mod math;
		/// The `Number` global object and related global methods
		pub mod number;
		/// The `Object` global object
		pub mod object;
		/// The `String` global object
		pub mod string;
		/// The global URI methods
		pub mod uri;
		/// An arbritary Javascript value
		pub mod value;
	}
}
/// The default backend implemented on top of LibJIT
pub mod back {
	/// The compiler, which transforms Javascript expressions to LibJIT IR
	pub mod compiler;
	/// The executor, which runs the LibJIT IR by compiling it then running it
	pub mod executor;
}
/// Javascript parsing and syntax
pub mod syntax {
	/// The Javascript Abstract Syntax Tree
	pub mod ast {
		/// Constants
		pub mod constant;
		/// Expressions
		pub mod expr;
		/// Keywords
		pub mod keyword;
		/// Operators
		pub mod op;
		/// Positions
		pub mod pos;
		/// Punctuators
		pub mod punc;
		/// Tokens
		pub mod token;
		/// An expression typer
		pub mod typer;
		/// Types
		pub mod types;
	}
	/// Parses a string stream into a sequence of tokens
	pub mod lexer;
	/// Parses a sequence of tokens into expressions
	pub mod parser;
}