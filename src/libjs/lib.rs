#![crate_id = "js"]
#![comment = "Javascript execution"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(
	html_favicon_url = "http://tombebbington.github.io/favicon.png",
	html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![experimental]
#![feature(phase, globs)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]

//! This crate provides a Javascript execution library with an
//! JITCompiler and a Javascript standard library.
extern crate collections;
extern crate syntax = "js_syntax";
extern crate rand;
extern crate serialize;
extern crate time;
extern crate url;
extern crate jit;
#[phase(syntax)]
extern crate jit_macro;
/// The execution engines
pub mod run {
	/// Defines the base executor trait which the execution engines derive from
	pub mod exec;
	/// Just-In-Time Compilation using libJIT
	pub mod jit;
}
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