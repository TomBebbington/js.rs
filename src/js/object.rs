use collections::treemap::TreeMap;
use js::function::Function;
use js::value::{Value, VFunction, VUndefined, VObject, VInteger, ResultValue};
use std::gc::Gc;
use std::fmt;

#[deriving(Clone)]
pub type ObjectData = TreeMap<~str, Value>;
macro_rules! js_object(
	($field:ident: $val:expr) => ( // invoke it like js_object(a: 40)
		match $inp {
			$sp(x) => { return x; }
			_ => {}
		}
	);
)
impl fmt::Show for ObjectData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		try!(f.buf.write_str("{ "));
		for (k, v) in self.iter() {
			try!(write!(f.buf, "{} = {}\n", k, v));
		}
		f.buf.write_str("}")
	}
}
/// Create new object
pub fn make_object(this:Value, _:Value, _:Vec<Value>) -> ResultValue {
	Ok(VUndefined)
}
/// Get the prototype
pub fn get_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let obj = args.get(0);
	return Ok(obj.get_field(~"__proto__").clone());
}
/// Set the prototype
pub fn set_proto_of(_:Value, _:Value, mut args:Vec<Value>) -> ResultValue {
	let proto = args.get(1).clone();
	let mut obj = args.get_mut(0);
	obj.set_field(~"__proto__", proto);
	return Ok(obj.clone());
}
/// Create a new 'Object' object
pub fn _create() -> Value {
	let mut func = Function::new(make_object, 0);
	let mut prototype : ObjectData = TreeMap::new();
	func.object.swap(~"length", VInteger(1));
	func.object.swap(~"prototype", VObject(Gc::new(prototype)));
	func.object.swap(~"setPrototypeOf", VFunction(Gc::new(Function::new(set_proto_of, 2))));
	func.object.swap(~"getPrototypeOf", VFunction(Gc::new(Function::new(get_proto_of, 1))));
	VFunction(Gc::new(func))
}