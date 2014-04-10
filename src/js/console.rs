use js::function::{NativeFunction, NativeFunc};
use js::value::{Value, ResultValue, VFunction, VUndefined, VObject, to_value, from_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::io::stdio::stderr;
use std::io::Writer;
use time::{now, strftime};
/// Print a javascript value to stdout
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<~str> = FromIterator::from_iter(args.iter().map(|x|from_value::<~str>(*x).unwrap()));
	println!("{}: {}", strftime("%X", &now()), args.connect(" "));
	return Ok(Gc::new(VUndefined));
}
/// Print a javascript value to stderr
pub fn error(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<~str> = FromIterator::from_iter(args.iter().map(|x|from_value::<~str>(*x).unwrap()));
	match writeln!(&mut stderr().unwrap(), "{}: {}", strftime("%X", &now()), args.connect(" ")) {
		Ok(_) => Ok(Gc::new(VUndefined)),
		Err(io_error) => Err(to_value(io_error.to_str()))
	}
}
/// Create a new 'console' object
pub fn _create() -> Value {
	let mut console = TreeMap::new();
	console.insert(~"log", to_value(log));
	console.insert(~"error", to_value(error));
	console.insert(~"exception", to_value(error));
	to_value(console)
}