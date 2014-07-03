#![crate_id = "github.com/TomBebbington/js.rs#js:0.1"]
#![comment = "Javascript parsing, compilation and execution using LibJIT"]
#![license = "MIT"]
#![crate_type = "lib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png"
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
pub mod front;
/// The default backend implemented on top of LibJIT
pub mod back;
/// Javascript parsing and syntax
pub mod syntax;