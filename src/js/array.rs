use collections::treemap::TreeMap;
use js::function::{NativeFunc, NativeFunction};
use js::value::{Value, VFunction, VUndefined, VInteger, ResultValue};
use std::gc::Gc;
use std::cell::RefCell;
pub type ObjectData = TreeMap<~str, Value>;

/// Create new object
pub fn make_array(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	this.borrow().set_field(~"length", Gc::new(VInteger(0)));
	Ok(Gc::new(VUndefined))
}
/// Create a new 'Object' object
pub fn _create() -> Value {
	let mut func = NativeFunction::new(make_array, 0);
	Gc::new(VFunction(RefCell::new(NativeFunc(func))))
}