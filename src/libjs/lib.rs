#![crate_id = "js"]
#![comment = "Javascript execution"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![experimental]

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

//! A Javascript execution library
extern crate collections;
extern crate syntax = "js_syntax";
extern crate rand;
extern crate serialize;
extern crate time;
extern crate url;
/// The interpreter
pub mod exec;
/// The standard Javascript library
pub mod stdlib {
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