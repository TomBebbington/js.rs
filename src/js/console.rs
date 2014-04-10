use js::function::{NativeFunction, NativeFunc};
use js::value::{Value, ResultValue, VFunction, VUndefined, VObject, to_value, from_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::iter::FromIterator;
use time::{now, strftime};
/// Print a javascript value to stdout
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	let args : Vec<~str> = FromIterator::from_iter(args.iter().map(|x|from_value::<~str>(*x).unwrap()));
	println!("{}: {}", strftime("%X", &now()), args.connect(" "));
	return Ok(Gc::new(VUndefined));
}
/// Create a new 'console' object
pub fn _create() -> Value {
	let mut console = TreeMap::new();
	console.insert(~"log", to_value(log));
	to_value(console)
}