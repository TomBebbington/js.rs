use js::function::{NativeFunction, NativeFunc};
use js::object::ObjectData;
use js::value::{Value, ResultValue, VFunction, VString, VObject, VUndefined};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;

/// Create a new error
pub fn make_error(this:Value, _:Value, args:Vec<Value>) -> ResultValue {
	if args.len() >= 1 {
		this.borrow().set_field(~"message", Gc::new(VString(args.get(0).borrow().to_str())));
	}
	Ok(Gc::new(VUndefined))
}
/// Get the string representation of the error
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	let name = this.borrow().get_field(~"name");
	let message = this.borrow().get_field(~"message");
	Ok(Gc::new(VString(format!("{}: {}", name.borrow(), message.borrow()))))
}
/// Create a new 'Error' object
pub fn _create() -> Value {
	let mut func = NativeFunction::new(make_error, 0);
	let mut prototype : ObjectData = TreeMap::new();
	prototype.insert(~"message", Gc::new(VString(~"")));
	prototype.insert(~"name", Gc::new(VString(~"Error")));
	prototype.insert(~"toString", Gc::new(VFunction(RefCell::new(NativeFunc(NativeFunction::new(to_string, 0))))));
	func.object.insert(~"prototype", Gc::new(VObject(RefCell::new(prototype))));
	Gc::new(VFunction(RefCell::new(NativeFunc(func))))
}