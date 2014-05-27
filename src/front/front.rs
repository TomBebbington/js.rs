#![crate_id = "js.rs"]
#![comment = "Javascript parsing and execution command line tool"]
#![license = "MIT"]
#![crate_type = "bin"]
#![doc(
	html_favicon_url = "http://tombebbington.github.io/favicon.png",
	html_root_url = "http://tombebbington.github.io/js.rs/"
)]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint, unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation, uppercase_variables, non_camel_case_types, unused_must_use)]

//! A Javascript execution command line tool

extern crate js;
extern crate syntax = "js_syntax";
extern crate getopts;
extern crate collections;
/// Interactive mode
pub mod interactive;
/// Unit test mode
pub mod tests;
/// Script runner mode
pub mod runner;
/// The main function
pub fn main() {
	let opts = [
		getopts::optflag("h", "help", "Show this message"),
		getopts::optflag("v", "verbose", "Enable verbose output"),
		getopts::optflag("t", "tests", "Run tests"),
		getopts::optflag("i", "interactive", "Run in interactive mode"),
		getopts::optopt("s", "source-code", "Run some Javascript code", "The path to the source code")
	];
	let m = getopts::getopts(std::os::args().as_slice(), opts).unwrap();
	match m.opt_str("s") {
		Some(path) => {
			runner::Runner::new(m).run(path)
		},
		None if m.opt_present("h") => {
			println!("{}", getopts::usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		},
		None if m.opt_present("t") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "test") => {
			tests::Tests::new(m).run();
		},
		None if m.opt_present("i") || (m.free.len() >= 2 && m.free.get(1).as_slice() == "interactive") => {
			interactive::Interactive::new(m).run();
		},
		None if m.free.len() >= 2 => {
			runner::Runner::new(m.clone()).run(m.free.get(1).clone());
		},
		None => {
			println!("{}", getopts::short_usage("Usage: js.rs [OPTIONS] [INPUT]", opts));
		}
	}
}