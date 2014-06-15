#![crate_id = "github.com/TomBebbington/js.rs"]
#![comment = "Javascript parsing and execution command line tool"]
#![license = "MIT"]
#![crate_type = "bin"]
#![doc(
	html_favicon_url = "http://tombebbington.github.io/favicon.png",
	html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]
#![feature(phase)]
//! A Javascript execution command line tool

extern crate back;
extern crate collections;
extern crate front;
extern crate getopts;
extern crate jit;
#[phase(plugin, link)]
extern crate log;
extern crate syntax = "js_syntax";
/// Interactive mode
pub use interactive::Interactive;
/// Unit test mode
pub use tests::Tests;
/// Script runner mode
pub use runner::Runner;
mod interactive;
mod tests;
mod runner;
/// The main function
pub fn main() {
	let opts = [
		getopts::optflag("h", "help", "Show this message"),
		getopts::optflag("t", "tests", "Run tests"),
		getopts::optflag("i", "interactive", "Run in interactive mode"),
		getopts::optopt("s", "source-code", "Run some Javascript code", "The path to the source code")
	];
	let m = getopts::getopts(std::os::args().as_slice(), opts).ok().expect("Could not parse arguments");
	match m.opt_str("s") {
		Some(path) => {
			Runner::new(path).run()
		},
		None if m.opt_present("h") => {
			println!("{}", getopts::usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		},
		None if m.opt_present("t") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "test") => {
			Tests::new().run();
		},
		None if m.opt_present("i") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "interactive") => {
			Interactive::new().run();
		},
		None if m.free.len() >= 2 => {
			Runner::new(m.free.get(1).clone()).run();
		},
		None => {
			println!("{}", getopts::short_usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		}
	}
}