#![crate_id = "github.com/TomBebbington/js.rs#front:0.1"]
#![comment = "Javascript backend"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png",
    html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![experimental]
#![feature(phase, macro_rules, globs)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]

//! This crate provides the base for Javascript execution
//! by including defining traits and the standard library.
extern crate collections;
extern crate syntax = "js_syntax";
extern crate rand;
extern crate serialize;
extern crate time;
extern crate url;
#[phase(plugin, link)]
extern crate log;
#[macro_escape]
/// A javascript value macro
pub mod macro;
/// The backend defining traits
pub mod run {
    /// Compilation of code
    pub mod compiler;
    /// Execution of compiled code
    pub mod executor;
}
/// The standard Javascript library
pub mod stdlib {
    /// The global `Array` object
    pub mod array;
    /// The global `Boolean` object
    pub mod boolean;
    /// The global `console` object
    pub mod console;
    /// The global `Error` object
    pub mod error;
    /// The global `Function` object and function value representations
    pub mod function;
    /// The global `JSON` object
    pub mod json;
    /// The global `Math` object
    pub mod math;
    /// The global `Number` object with related functions and constants
    pub mod number;
    /// The global `Object` object
    pub mod object;
    /// The global `String` object
    pub mod string;
    /// Contains global methods concerning URIs
    pub mod uri;
    /// Javascript values, utility methods and conversion between Javascript values and Rust values
    pub mod value;
}