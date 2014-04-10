use collections::treemap::TreeMap;
use js::function::{NativeFunc, NativeFunction};
use js::value::{Value, VFunction, VUndefined, VInteger, ResultValue, to_value};
use std::gc::Gc;
use std::cell::RefCell;
pub type ObjectData = TreeMap<~str, Value>;

/// Create new array
pub fn make_array(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	this.borrow().set_field(~"length", to_value(0i32));
	Ok(Gc::new(VUndefined))
}
/// Create a new 'Array' object
pub fn _create() -> Value {
	let mut func = NativeFunction::new(make_array);
	Gc::new(VFunction(RefCell::new(NativeFunc(func))))
}