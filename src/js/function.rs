use collections::treemap::TreeMap;
use js::object::ObjectData;
use js::value::{Value, VFunction, VInteger, VObject, ResultValue};
use std::gc::Gc;
type FunctionData = fn(Value, Value, Vec<Value>) -> ResultValue;
/// Represents a Javascript function in-memory
pub struct Function {
	/// The fields associated with the function
	pub object : ObjectData,
	/// The callable function data
	pub data: FunctionData,
	/// The number of arguments
	pub nargs: uint
}
impl Function {
	/// Make a new function with the given function data
	pub fn new(data : FunctionData, nargs: uint) -> Function {
		let mut obj = TreeMap::new();
		obj.insert(~"arguments", VInteger(nargs as i32));
		Function {object: obj, data: data, nargs: nargs}
	}
	/// Call a function with some arguments
	pub fn call(&self, this:Value, callee:Value, args:Vec<Value>) -> ResultValue {
		let func = self.data;
		func(this, callee, args)
	}
}
impl Eq for Function {
	fn eq(&self, b : &Function) -> bool {
		self == b
	}
}
impl Clone for Function {
	fn clone(&self) -> Function {
		Function{ object: self.object.clone(), data: self.data.clone(), nargs: self.nargs}
	}
}
/// Create a new 'Function' object
pub fn _create() -> Value {
	let mut function = TreeMap::new();
	VObject(Gc::new(function))
}