use collections::treemap::TreeMap;
use js::function::Function;
use js::value::{Value, VFunction, VUndefined, VInteger, ResultValue};
use std::gc::Gc;
pub type ObjectData = TreeMap<~str, Value>;

/// Create new object
pub fn make_array(mut this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	this.set_field(~"length", VInteger(0));
	Ok(VUndefined)
}
/// Create a new 'Object' object
pub fn _create() -> Value {
	let mut func = Function::new(make_array, 0);
	VFunction(Gc::new(func))
}