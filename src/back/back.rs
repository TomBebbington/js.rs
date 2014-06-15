#![crate_id = "github.com/TomBebbington/js.rs#back:0.1"]
#![comment = "Javascript compilation and execution using LibJIT"]
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

//! This crate provides a Javascript execution backend
//! using LibJIT.
extern crate front;
extern crate syntax = "js_syntax";
extern crate jit;
pub use compiler::JitCompiler;
pub use executor::JitExecutor;
mod compiler;
mod executor;