use js::function::{NativeFunction, NativeFunc};
use js::value::{Value, ResultValue, VFunction, VUndefined, VObject, to_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use time::{now, strftime};
/// Print a javascript value to stdout
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	println!("{}: {}", strftime("%X", &now()), args.get(0).borrow());
	return Ok(Gc::new(VUndefined));
}
/// Create a new 'console' object
pub fn _create() -> Value {
	let mut console = TreeMap::new();
	console.insert(~"log", to_value(log));
	to_value(console)
}