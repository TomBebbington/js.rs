use js::function::{NativeFunc, NativeFunction};
use js::value::{Value, VFunction, VUndefined, VObject, VInteger, VString, ResultValue, to_value};
use collections::treemap::TreeMap;
use std::gc::Gc;
use std::cell::RefCell;
use std::fmt;

#[deriving(Clone)]
pub type ObjectData = TreeMap<~str, Value>;
impl fmt::Show for ObjectData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(f.buf.write_str("{ "));
		for (k, v) in self.iter() {
			try!(write!(f.buf, "{} = {}\n", k, v.borrow()));
		}
		f.buf.write_str("}")
	}
}
/// Create new object
pub fn make_object(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(Gc::new(VUndefined))
}
/// Get the prototype
pub fn get_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	return Ok(obj.borrow().get_field(~"__proto__"));
}
/// Set the prototype
pub fn set_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let proto = args.get(1).clone();
	let obj = args.get(0);
	obj.borrow().set_field(~"__proto__", proto);
	return Ok(*obj);
}
/// To string
pub fn to_string(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	return Ok(Gc::new(VString(this.borrow().to_str())));
}
/// Create a new 'Object' object
pub fn _create() -> Value {
	let mut func = NativeFunction::new(make_object, 0);
	let mut prototype : ObjectData = TreeMap::new();
	prototype.insert(~"toString", to_value(to_string));
	func.object.insert(~"length", to_value(1i32));
	func.object.insert(~"prototype", to_value(prototype));
	func.object.insert(~"setPrototypeOf", to_value(set_proto_of));
	func.object.insert(~"getPrototypeOf", to_value(get_proto_of));
	Gc::new(VFunction(RefCell::new(NativeFunc(func))))
}