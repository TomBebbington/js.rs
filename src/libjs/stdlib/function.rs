use stdlib::object::{ObjectData, Property};
use stdlib::value::{Value, ResultValue, to_value};
use collections::treemap::TreeMap;
use jit;
pub type NativeFunctionData = fn(Value, Value, Vec<Value>) -> ResultValue;
#[deriving(Clone)]
/// A Javascript function
pub enum Function {
	/// A native javascript function
	NativeFunc(NativeFunction),
	/// A regular javascript function
	RegularFunc(RegularFunction)
}
#[deriving(Clone)]
/// Represents a regular javascript function in memory
pub struct RegularFunction {
	/// The fields associated with the function
	pub object : ObjectData,
	/// This function's JIT representation
	pub jit : jit::Function,
	/// The argument names of the function
	pub args : Vec<String>
}
impl RegularFunction {
	/// Make a new regular function
	pub fn new(jit : jit::Function, args: Vec<String>) -> RegularFunction {
		let mut obj = TreeMap::new();
		obj.insert("arguments".into_string(), Property::new(to_value(args.len() as i32)));
		RegularFunction {object: obj, jit: jit, args: args}
	}
}
#[deriving(Clone)]
/// Represents a native javascript function in memory
pub struct NativeFunction {
	/// The fields associated with the function
	pub object : ObjectData,
	/// The callable function data
	pub data: NativeFunctionData
}
impl NativeFunction {
	/// Make a new native function with the given function data
	pub fn new(data : NativeFunctionData) -> NativeFunction {
		let obj = TreeMap::new();
		NativeFunction {object: obj, data: data}
	}
}
/// Create a new `Function` object
pub fn _create(_ : Value) -> Value {
	let function : ObjectData = TreeMap::new();
	to_value(function)
}
/// Initialise the global object with the `Function` object
pub fn init(global:Value) {
	global.set_field_slice("Function", _create(global));
}