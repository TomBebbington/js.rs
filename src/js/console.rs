use js::function::Function;
use collections::treemap::TreeMap;
use js::object::ObjectData;
use js::value::{Value, ResultValue, VFunction, VUndefined, VObject};
use std::gc::Gc;
use time::{now, strftime};
/// Print a javascript value to stdout
pub fn log(_:Value, _:Value, args:Vec<Value>) -> ResultValue {
	println!("{}: {}", strftime("%X", &now()), args.get(0).to_str());
	return Ok(VUndefined);
}
/// Create a new 'console' object
pub fn _create() -> Value {
	let mut console = TreeMap::new();
	console.insert(~"log", VFunction(Function::new(log, 1)));
	VObject(console)
}